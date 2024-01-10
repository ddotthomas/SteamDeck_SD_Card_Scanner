use crate::scanning::{self, Card};
use iced::widget::{column, container, row, text};
use iced::{executor, Alignment, Application, Command, Element, Length, Subscription, Theme};

mod controller;
mod pages;
mod theming;
pub mod utils;

use pages::Page;
use utils::control_button;

use self::controller::ControlEvent;

pub struct App {
    pages: Vec<Page>,
    current_page: usize,
    card_data: Vec<Card>,
    search_term: String,
    // Tracks how far to the right the selection should be
    selected_x: usize,
    // Tracks how far up the selection should be
    selected_y: usize,
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
                current_page: 0,
                card_data: scanning::get_card_data(),
                search_term: String::new(),
                selected_x: 0,
                selected_y: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Steam Deck SD Card Scanner")
    }

    fn update(&mut self, event: Message) -> Command<Self::Message> {
        match event {
            Message::ScanCard => crate::scanning::update_list(&mut self.card_data),
            Message::Exit => std::process::exit(0),
            Message::SearchInput(text_input) => self.search_term = text_input,
            Message::Settings => self.current_page = 1,
            Message::Home => self.current_page = 0,
            Message::ChangeCardName(card_name, card_uuid) => {
                self.card_data = utils::change_card_name(card_name, card_uuid, &self.card_data)
            }
            Message::ControllerEvent(controller_event) => {
                match controller_event {
                    ControlEvent::Left => {
                        self.selected_x = self.selected_x.saturating_sub(1);
                    }
                    ControlEvent::Right => {
                        self.selected_x += 1;
                    }
                    ControlEvent::Down => {
                        self.selected_y = self.selected_y.saturating_sub(1);
                    }
                    ControlEvent::Up => {
                        self.selected_y += 1;
                    }
                    ControlEvent::Search => {} //TODO, select the Search Box
                    ControlEvent::Back => {}   //TODO, go back
                    ControlEvent::Select => {} //TODO, select the current highlight
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        controller::read_controller().map(Message::ControllerEvent)
    }

    fn view(&self) -> Element<Message> {
        let mut controls = vec![container(text("Steam Deck\nSD Card Scanner").size(50))
            .padding(10)
            .into()];

        if self.current_page != 0 {
            controls.push(control_button("Home", Message::Home));
        }
        if self.current_page != 1 {
            controls.push(control_button("Settings", Message::Settings));
        }
        controls.push(control_button("Rescan Card", Message::ScanCard));
        controls.push(control_button("Exit", Message::Exit));

        let controls_column = column(controls).padding(12).align_items(Alignment::Center);

        let content = self.pages[self.current_page].view(&self.card_data, &self.search_term);

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
    SearchInput(String),
    Settings,
    Home,
    ChangeCardName(String, String),
    ControllerEvent(controller::ControlEvent),
}
