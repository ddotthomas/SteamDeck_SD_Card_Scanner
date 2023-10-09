use crate::scanning::Card;
use iced::widget::{column, container, row, text};
use iced::{executor, window, Alignment, Application, Command, Element, Length, Theme};

mod pages;
mod theming;
mod utils;

use pages::Page;
use utils::control_button;

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
                pages: vec![Page::List, Page::Settings],
                current: 0,
                card_data: utils::get_card_data(),
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
            Message::Exit => std::process::exit(0),
            Message::Fullscreen => return window::resize(1280, 800),
            Message::SearchInput(text_input) => self.search_term = text_input,
            Message::Settings => self.current = 1,
            Message::Home => self.current = 0,
            Message::ChangeCardName(card_name, card_uuid) => {
                self.card_data = utils::change_card_name(card_name, card_uuid, &self.card_data)
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut controls = vec![container(text("Steam Deck\nSD Card Scanner").size(50))
            .padding(10)
            .into()];

        if self.current != 0 {
            controls.push(control_button("Home", Message::Home));
        }
        if self.current != 1 {
            controls.push(control_button("Settings", Message::Settings));
        }
        controls.push(control_button("Rescan Card", Message::ScanCard));
        controls.push(control_button("Exit", Message::Exit));
        controls.push(control_button("Fullscreen", Message::Fullscreen));

        let controls_column = column(controls).padding(12).align_items(Alignment::Center);

        let content = self.pages[self.current].view(&self.card_data, &self.search_term);

        container(row!(controls_column, content))
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
    Settings,
    Home,
    ChangeCardName(String, String),
}
