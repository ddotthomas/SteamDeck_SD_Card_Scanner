use crate::scanning::{Card, Game};
use iced::widget::Column;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{executor, window, Alignment, Application, Command, Element, Length, Theme};
use std::process;

pub struct App {
    pages: Vec<Page>,
    current: usize,
    card_data: Vec<Card>,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (App, iced::Command<Message>) {
        (
            App {
                pages: vec![Page::List],
                current: 0,
                card_data: get_card_data(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Steam Deck SD Card Scanner")
    }

    fn update(&mut self, event: Message) -> Command<Self::Message> {
        match event {
            Message::ScanCard => crate::scanning::add_current_card(&mut self.card_data),
            Message::Exit => process::exit(0),
            Message::Fullscreen => return window::toggle_maximize(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let controls = column(vec![
            container(text("Steam Deck\nSD Card Scanner").size(50))
                .padding(10)
                .into(),
            container(
                button(text("Rescan Card").size(33))
                    .padding(12)
                    .on_press(Message::ScanCard),
            )
            .padding(10)
            .into(),
            container(
                button(text("Exit").size(33))
                    .padding(12)
                    .on_press(Message::Exit),
            )
            .padding(10)
            .into(),
            //container(
            //    button(text("Fullscreen").size(33))
            //        .padding(12)
            //        .on_press(Message::Fullscreen),
            //)
            //.padding(10)
            //.into(),
        ])
        .padding(12)
        .align_items(Alignment::Center);

        let content = self.pages[self.current].view(&self.card_data);

        container(row!(controls, scrollable(content)))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    /// Activates the scan card function manually, called when 'Scan Card' is clicked
    ScanCard,
    /// Exit the application, called when 'Exit' is clicked
    Exit,
    /// Attempt at a fullscreen button and to fix the issue with the app's resolution while on the desktop and in game mode
    Fullscreen,
}

pub enum Page {
    List,
}

impl<'a> Page {
    fn view(&'a self, card_data: &'a Vec<Card>) -> Element<Message> {
        match self {
            Page::List => Self::list(card_data).into(),
        }
    }

    fn list(list: &'a Vec<Card>) -> Column<'a, Message> {
        column(create_card_and_games_list(list)).width(Length::Fill)
    }
}

/// Converts the list data into an Iced GUI list of the cards and their games
fn create_card_and_games_list(list: &Vec<Card>) -> Vec<Element<Message>> {
    let mut return_list: Vec<Element<Message>> = vec![];

    for card in list {
        // List is seperated by different SD cards
        return_list.push(text(format!("Card: {}", card.name)).size(50).into());

        if card.heroic.is_some() || card.lutris.is_some() {
            // Label the Steam Library if there's also Non Steam Libraries
            return_list.push(text(format!("Steam Games")).size(40).into())
        }

        return_list.push(
            //
            card.games
                .iter()
                .fold(column![], |column: Column<Message>, game: &Game| {
                    column.push(text(format!("{}", game.name)).size(30))
                })
                .into(),
        );

        if card.lutris.is_some() {
            return_list.push(text(format!("Lutris Library")).size(40).into());

            if let Some(library) = &card.lutris {
                return_list.push(
                    library
                        .games
                        .iter()
                        .fold(column![], |column: Column<Message>, game: &Game| {
                            column.push(text(format!("{}", game.name)).size(30))
                        })
                        .into(),
                )
            }
        }

        if card.heroic.is_some() {
            return_list.push(text(format!("Heroic Library")).size(40).into());

            if let Some(library) = &card.heroic {
                return_list.push(
                    library
                        .games
                        .iter()
                        .fold(column![], |column: Column<Message>, game: &Game| {
                            column.push(text(format!("{}", game.name)).size(30))
                        })
                        .into(),
                )
            }
        }
    }

    // return_list.push(text("For some reason the either the iced-rs backend or Gamescope doesn't allow the mouse to move down here.\nIf you know why or can help at all consider contributing on GitHub").vertical_alignment(alignment::Vertical::Bottom).size(15).into());

    return_list
}

/// Gets the saved data from the json file in ~/.config and updates it with the currently inserted card
fn get_card_data() -> Vec<Card> {
    let list: Vec<Card> = match crate::scanning::get_saved_data() {
        Some(mut list) => {
            // Update the current cards data to the list, update the file
            crate::scanning::add_current_card(&mut list);
            crate::scanning::save_data(&list);
            list
        }
        None => match crate::scanning::scan_card(None) {
            // If the save file wasn't found, scan the current card if it exists and create the save file
            Some(card) => {
                crate::scanning::save_data(&vec![card.clone()]);
                vec![card]
            }
            None => {
                eprintln!("Error scanning SD card data");
                // If there wasn't any saved data found in ~/.config/sdscannerssave.json and no current SD card, return an empty list
                vec![]
            }
        },
    };

    list
}
