use iced::widget::{button, column, container, row, scrollable, text};
use iced::widget::{Button, Column, Container};
use iced::{window, Alignment, Color, Element, Length, Renderer, Sandbox, Settings, Theme};

mod scanner;

use scanner::{Card, Game};

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
    theme: Theme,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {
        App {
            pages: vec![Page::List(vec![])],
            current: 0,
            theme: Theme::Dark,
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
        }
    }

    fn view(&self) -> Element<Message> {
        let mut controls = iced::widget::column(vec![
            text("Steam Deck\nSD Card Scanner").size(30).into(),
            button("Test List").on_press(Message::TestList).into(),
            button("Clear List").on_press(Message::ClearList).into(),
        ])
        .padding(20)
        .align_items(Alignment::Center);
        let content = self.pages[self.current].view();
        container(row!(controls, scrollable(content))).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ClearList,
    TestList,
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
                games: vec![Game {
                    name: String::from("Test Game"),
                }],
            }),
        }
    }

    fn clear_list(&mut self) {
        match self {
            Page::List(list) => list.clear(),
        }
    }
}

fn create_card_list(list: &Vec<Card>) -> Vec<Element<Message>> {
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
