use dirs;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::widget::{Button, Column, Container};
use iced::{window, Alignment, Color, Element, Length, Renderer, Sandbox, Settings, Theme};
use std::{fs, path::PathBuf, collections::HashMap};

mod scanning;

use scanning::{Card, Game};

fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings { 
            size: (1280, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

pub struct App {
    pages: Vec<Page>,
    current: usize,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {
        App {
            pages: vec![Page::List(get_card_data())],
            current: 0,
        }
    }

    fn title(&self) -> String {
        format!("Steam Deck SD Card Scanner")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::ClearList => {
                Page::clear_list(&mut self.pages[self.current]);
            }
            Message::TestList => {
                Page::test_list(&mut self.pages[self.current]);
            }
            Message::ScanCard => Page::scan_card(&mut self.pages[self.current]),
        }
    }

    fn view(&self) -> Element<Message> {
        let mut controls = iced::widget::column(vec![
            text("Steam Deck\nSD Card Scanner").into(),
            button("Scan Card").on_press(Message::ScanCard).into(),
            button("Test List").on_press(Message::TestList).into(),
            button("Clear List").on_press(Message::ClearList).into(),
        ])
        .padding(20)
        .align_items(Alignment::Center);
        let content = self.pages[self.current].view();
        container(row!(controls, scrollable(content))).into()
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
        column(create_card_list(list)).width(Length::Fill)
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

fn create_card_list(list: &Vec<Card>) -> Vec<Element<Message>> {
    // Creates the list of cards and their games underneath each one for the iced GUI
    let mut return_list: Vec<Element<Message>> = vec![];

    for card in list {
        return_list.push(text(format!("Card: {}", card.name)).size(30).into());

        let game_list =
            card.games
                .iter()
                .fold(column![], |column: Column<Message>, game: &Game| {
                    column.push(text(format!("Game: {}", game.name)))
                });

        return_list.push(game_list.into())
    }

    return_list
}

fn get_card_data() -> Vec<Card> {
    let list: Vec<Card> = match scanning::get_saved_data() {
        // Check for a json file save, add the data from it
        Some(mut list) => {
            scanning::add_current_card(&mut list);
            list
        } // Update the current cards data to the list, update the file
        None => match scanning::scan_card(None) {
            // If the save file wasn't found, scan the current card if it exists and create the save file
            Some(card) => vec![card],
            None => {
                eprintln!("Error scanning SD card data");
                vec![]
            },
        },
    };

    list
}



