use crate::scanning::{self, Card};
use iced::widget::{column, container, row, text, Container};
use iced::{executor, Alignment, Application, Command, Element, Length, Subscription, Theme};

mod controller;
mod pages;
mod theming;
pub mod utils;

use pages::Page;
use utils::{control_button, Selectable};

use self::controller::ControlEvent;
use self::utils::SelectCoords;

pub struct App {
    pages: Vec<Page>,
    current_page: usize,
    card_data: Vec<Card>,
    search_term: String,
    /// Struct to track the current selected UI element
    select_coords: SelectCoords,
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
                select_coords: (0, 0).into(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Steam Deck SD Card Scanner".to_string()
    }

    fn update(&mut self, event: Message) -> Command<Self::Message> {
        match event {
            Message::Exit => std::process::exit(0),
            Message::SearchInput(text_input) => {
                self.search_term = text_input;
                self.select_coords.y = 0;
            }
            Message::Settings => self.current_page = 1,
            Message::Home => self.current_page = 0,
            Message::ChangeCardName(card_name, card_uuid) => {
                self.card_data = utils::change_card_name(card_name, card_uuid, &self.card_data)
            }
            Message::ControllerEvent(controller_event) => {
                match controller_event {
                    ControlEvent::Left => self.move_left(),
                    ControlEvent::Right => self.move_right(),
                    ControlEvent::Down => self.move_down(),
                    ControlEvent::Up => self.move_up(),
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
        let mut controls: Vec<Element<Message>> =
            vec![container(text("Steam Deck\nSD Card Scanner").size(50))
                .padding(10)
                .into()];

        // List of buttons, seperated from other UI elements to be selected
        let mut button_panel: Vec<Container<Message>> = vec![];

        if self.current_page != 0 {
            button_panel.push(control_button("Home", Message::Home));
        }
        if self.current_page != 1 {
            button_panel.push(control_button("Settings", Message::Settings));
        }
        button_panel.push(control_button("Exit", Message::Exit));

        // If the selection is to the left, apply the highlight to the correct button
        if self.select_coords.x == 0 {
            // Check that y didn't exceed the bound of the list
            let index = self.select_coords.y.min(button_panel.len() - 1);
            let button = button_panel.remove(index).highlight();
            button_panel.insert(index, button);
        }

        controls.append(
            &mut button_panel
                .into_iter()
                .map(Element::from)
                .collect::<Vec<Element<Message>>>(),
        );

        let controls_column = column(controls).padding(12).align_items(Alignment::Center);

        let content = self.pages[self.current_page].view(self);

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
    /// Exit the application, called when 'Exit' is clicked
    Exit,
    SearchInput(String),
    Settings,
    Home,
    ChangeCardName(String, String),
    ControllerEvent(controller::ControlEvent),
}
