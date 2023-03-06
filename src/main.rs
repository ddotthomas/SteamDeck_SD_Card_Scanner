use iced::widget::{button, column, container, row, scrollable, text};
use iced::widget::Column;
use iced::{
    executor, window, Alignment, Application, Command, Element, Length, Settings, Theme,
};
use std::process;

mod scanning;

use scanning::{Card, Game};

fn main() -> iced::Result {
    let size = (1280, 800);
    App::run(Settings {
        window: window::Settings {
            size,
            // max_size: Some((1280, 800)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

pub struct App {
    pages: Vec<Page>,
    current: usize,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (App, iced::Command<Message>) {
        (
            App {
                pages: vec![Page::List(get_card_data())],
                current: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Steam Deck SD Card Scanner")
    }

    fn update(&mut self, event: Message) -> Command<Self::Message> {
        match event {
            Message::ClearList => {
                Page::clear_list(&mut self.pages[self.current]);
            }
            Message::TestList => {
                Page::test_list(&mut self.pages[self.current]);
            }
            Message::ScanCard => Page::scan_card(&mut self.pages[self.current]),
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
            //container(button(text("Test List").size(33)).padding(12).on_press(Message::TestList))
            //    .padding(10)
            //    .into(),
            //container(
            //    button(text("Clear List").size(33))
            //        .padding(12)
            //        .on_press(Message::ClearList),
            //)
            //.padding(10)
            //.into(),
            container(button(text("Exit").size(33)).padding(12).on_press(Message::Exit))
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

        let content = self.pages[self.current].view();

        container(row!(controls, scrollable(content))).width(Length::Fill).height(Length::Fill).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ClearList,
    TestList,
    ScanCard,
    Exit,
    Fullscreen,
}

pub enum Page {
    List(Vec<Card>),
}

impl<'a> Page {
    fn view(&self) -> Element<Message> {
        match self {
            Page::List(list) => Self::list(list).into(),
        }
    }

    fn list(list: &'a Vec<Card>) -> Column<'a, Message> {
        column(create_card_and_games_list(list)).width(Length::Fill)
    }

    fn test_list(&mut self) {
        match self {
            Page::List(list) => list.push(Card {
                uuid: String::from("000-00"),
                name: String::from("Card 1"),
                games: vec![
                    Game {
                        name: String::from("Test Game"),
                    },
                    Game {
                        name: String::from("Game Two"),
                    },
                ],
                lutris: None,
                heroic: None,
            }),
        }
    }

    fn clear_list(&mut self) {
        match self {
            Page::List(list) => list.clear(),
        }
    }

    fn scan_card(&mut self) {
        match self {
            Page::List(list) => scanning::add_current_card(list),
        }
    }
}

/// Creates the list of cards and their games underneath each one for the iced GUI
fn create_card_and_games_list(list: &Vec<Card>) -> Vec<Element<Message>> {
    let mut return_list: Vec<Element<Message>> = vec![];

    for card in list {
        return_list.push(text(format!("Card: {}", card.name)).size(50).into());

        // Label the Steam Library if there's also Non Steam Libraries
        if card.heroic.is_some() || card.lutris.is_some() {
            return_list.push(text(format!("Steam Games")).size(40).into())
        }

        return_list.push(
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
    let list: Vec<Card> = match scanning::get_saved_data() {
        Some(mut list) => {
            // Update the current cards data to the list, update the file
            scanning::add_current_card(&mut list);
            scanning::save_data(&list);
            list
        }
        None => match scanning::scan_card(None) {
            // If the save file wasn't found, scan the current card if it exists and create the save file
            Some(card) => {
                scanning::save_data(&vec![card.clone()]);
                vec![card]
            }
            None => {
                eprintln!("Error scanning SD card data");
                vec![]
            }
        },
    };

    list
}
