use crate::scanning::{Card, Game};
use iced::widget::{
    button, column, container, row, scrollable, text, text_input, Column, Scrollable,
};
use iced::{executor, window, Alignment, Application, Command, Element, Length, Theme};
use std::process;

mod theming;
mod utils;

pub struct App {
    pages: Vec<Page>,
    current: usize,
    card_data: Vec<Card>,
    search_term: String,
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
                search_term: String::new(),
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
            Message::SearchInput(text_input) => self.search_term = text_input,
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

        let content = self.pages[self.current].view(&self.card_data, &self.search_term);

        container(row!(controls, content))
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
    SearchInput(String),
}

pub enum Page {
    List,
    // Settings,
}

impl<'a> Page {
    /// view() probably shouldn't be designed to require card_data or search_term but it works for now
    fn view(&'a self, card_data: &'a Vec<Card>, search_term: &'a str) -> Element<Message> {
        match self {
            Page::List => Self::list(card_data, search_term).into(),
            // Page::Settings => Self::settings(card_data).into(),
        }
    }

    fn list(list: &'a Vec<Card>, search_term: &'a str) -> Column<'a, Message> {
        let mut element_list: Vec<Element<Message>> = vec![row(vec![
            text_input("Filter Search...", search_term, |text_value| {
                Message::SearchInput(text_value)
            })
            .size(30)
            .width(Length::FillPortion(2))
            .padding(4)
            .into(),
            text(format!(
                "Current Card: {}",
                if let Some(card_name) = utils::get_card_name(list) {
                    card_name
                } else {
                    String::from("No Card Detected")
                }
            ))
            .size(30)
            .width(Length::FillPortion(3))
            .into(), // TODO, make it translate the current UUID into the user's name for the card
        ])
        .into()];

        element_list.push(create_card_and_games_list(list, search_term).into());
        column(element_list).width(Length::Fill)
    }

    // TODO
    // fn settings(list_data: &Vec<Card>) -> Column<Message> {
    //     let mut element_list: Vec<Element<Message>> = vec![];
    // 
    //     todo!()
    // }
}

/// Converts the list data into an Iced GUI list of the cards and their games
/// Also provides the search functionality by filtering the list data by the `search_term`
/// The `search_term` is is provided by the user in search bar
fn create_card_and_games_list<'a>(
    list: &'a Vec<Card>,
    search_term: &'a str,
) -> Scrollable<'a, Message> {
    let mut return_list: Vec<Element<Message>> = vec![];

    let list = if search_term.is_empty() {
        // If the search term is empty, don't filter the list
        list.clone()
    } else {
        utils::filter_list(list, search_term)
    };

    for card in list {
        // List is seperated by different SD cards

        // If the lists of games on the card are empty and the search_term isn't empty
        // We'll assume the card has been 'filtered out' and wont display it at all
        if card.games.is_empty()
            && match &card.heroic {
                None => true,
                Some(heroic_list) => heroic_list.games.is_empty(),
            }
            && match &card.lutris {
                None => true,
                Some(lutris_list) => lutris_list.games.is_empty(),
            }
            && !search_term.is_empty()
        {
            continue;
        }

        return_list.push(text(format!("Card: {}", card.name)).size(50).into());

        if !card.games.is_empty() {
            if card.heroic.is_some() || card.lutris.is_some() {
                // Label the Steam Library if there's also Non Steam Libraries
                return_list.push(
                    container(
                        text(format!("Steam Games"))
                            .style(theming::STEAM_COLOR)
                            .size(40),
                    )
                    .into(),
                );
                return_list.push(
                    container(text(""))
                        .width(Length::Units(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Units(theming::DIVIDER_BAR_HEIGHT))
                        .padding(4)
                        .style(theming::STEAM_CONTAINER_STYLE)
                        .into(),
                );
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
        }

        if let Some(library) = card.lutris {
            if !library.games.is_empty() {
                // If there were no games found or they were all filtered out by the search,
                // don't add any elements to differentiate the other libraries
                return_list.push(
                    text(format!("Lutris Library"))
                        .style(theming::LUTRIS_COLOR)
                        .size(40)
                        .into(),
                );

                // Add a divider line under the Lutris Library label
                return_list.push(
                    container(text(""))
                        .width(Length::Units(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Units(theming::DIVIDER_BAR_HEIGHT))
                        .padding(4)
                        .style(theming::LUTRIS_CONTAINER_STYLE)
                        .into(),
                );
                return_list.push(
                    library
                        .games
                        .iter()
                        .fold(column![], |column: Column<Message>, game: &Game| {
                            column.push(text(format!("{}", game.name)).size(30))
                        })
                        .into(),
                );
            }
        }

        if let Some(library) = card.heroic {
            if !library.games.is_empty() {
                return_list.push(
                    text(format!("Heroic Library"))
                        .style(theming::HEROIC_COLOR)
                        .size(40)
                        .into(),
                );
                return_list.push(
                    container(text(""))
                        .width(Length::Units(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Units(theming::DIVIDER_BAR_HEIGHT))
                        .padding(4)
                        .style(theming::HEROIC_CONTAINER_STYLE)
                        .into(),
                );
                return_list.push(
                    library
                        .games
                        .iter()
                        .fold(column![], |column: Column<Message>, game: &Game| {
                            column.push(text(format!("{}", game.name)).size(30))
                        })
                        .into(),
                );
            }
        }
    }

    scrollable(column(return_list).width(Length::Fill))
}

/// Gets the saved data from the json file in ~/.config and updates it with the currently inserted card
fn get_card_data() -> Vec<Card> {
    let list: Vec<Card> = match crate::scanning::get_saved_data() {
        Some(mut list) => {
            // Update the current cards data to the list, update the file
            crate::scanning::add_current_card(&mut list);
            crate::scanning::save_data_to_json(&list);
            list
        }
        None => match crate::scanning::scan_card(None) {
            // If the save file wasn't found, scan the current card if it exists and create the save file
            Some(card) => {
                crate::scanning::save_data_to_json(&vec![card.clone()]);
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
