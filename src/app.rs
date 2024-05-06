//! ## App
//! 
//! The App module is used to set up the Iced application and its helping modules `pages` and `theming`

use crate::scanning::{self, Card};
use iced::widget::{column, container, row, text};
use iced::{executor, window, Alignment, Application, Command, Element, Length, Theme, Size};

mod pages;
mod theming;
pub mod utils;

use pages::Page;
use utils::control_button;

/// `App` is used as the base holding all the data used in the application. 
// Use the values to change parts of the app and allow user control
pub struct App {
    /// List of Page enum variants, the `current` variable selects which one is selected.
    // The selected page variable could probably be changed just be a modifiable Page variant ie `page: Page`
    pages: Vec<Page>,
    /// Current selected page from the list of pages
    current: usize,
    /// The list of cards that were scanned, including their game list and some indentifying information
    card_data: Vec<Card>,
    /// The current phrase or word in the search bar
    search_term: String,
}

// Set-up the App type to be used as an Iced Application
// Must implement a couple different required methods that control how the GUI looks and works
impl Application for App {
    // Set the Application Message to be a variant of our local Message type
    type Message = Message;
    // Use the default async executor
    type Executor = executor::Default;
    // Use iced's default Themes, variant selected in the theme() method
    type Theme = Theme;
    // ?
    type Flags = ();

    // Set up our application with starting data
    fn new(_flags: ()) -> (App, iced::Command<Message>) {
        (
            App {
                // Add the different pages to be displayed in the view to the right
                pages: vec![Page::List, Page::Settings],
                // Set the current page to the first one, List
                current: 0,
                // Get the current card data by running get_card_data()
                card_data: scanning::get_card_data(),
                // Start the search as an empty string
                search_term: String::new(),
            },
            // Don't run any command upon the app starting up
            Command::none(),
        )
    }

    // Set the App's name in the titlebar
    fn title(&self) -> String {
        format!("Steam Deck SD Card Scanner")
    }

    // This method is called from buttons and other user input passing a Message with info on the input
    fn update(&mut self, event: Message) -> Command<Self::Message> {
        // Check what Message was passed
        match event {
            // Force a scan of the current card
            Message::ScanCard => crate::scanning::update_list(&mut self.card_data),
            // Exit the program with no error
            Message::Exit => std::process::exit(0),
            // Set the window to the Steam Deck's default size, another failed attempt to fix the mouse bug
            Message::Fullscreen => return window::resize(iced::window::Id::MAIN, Size::new(1280.0, 800.0)),
            // Set the App's search_term to whatever input was added to the search bar
            Message::SearchInput(text_input) => self.search_term = text_input,
            // Set the current page to the Settings page
            Message::Settings => self.current = 1,
            // Set the current page to the List page
            Message::Home => self.current = 0,
            // Change a card's displayed name
            Message::ChangeCardName(card_name, card_uuid) => {
                self.card_data = utils::change_card_name(card_name, card_uuid, &self.card_data)
            }
        }

        // Don't run any iced command if none were returned
        Command::none()
    }

    // Set up the GUI display
    fn view(&self) -> Element<Message> {

        // Set up the list of control buttons on the apps left side
        let mut controls = vec![container(text("Steam Deck\nSD Card Scanner").size(50))
            .padding(10)
            .into()];

        // If the current page isn't already on List, add a Home button
        if self.current != 0 {
            controls.push(control_button("Home", Message::Home));
        }
        // If we're not on the settings page, add a Settings button
        if self.current != 1 {
            controls.push(control_button("Settings", Message::Settings));
        }
        // Add a scan card, exit, and fullscreen button
        controls.push(control_button("Rescan Card", Message::ScanCard));
        controls.push(control_button("Exit", Message::Exit));
        controls.push(control_button("Fullscreen", Message::Fullscreen));

        // Put the control buttons in a vertical column adding padding to the items
        let controls_column = column(controls).padding(12).align_items(Alignment::Center);

        // Set the content in the right side view depending on the current app state, calling view() on the current Page variant
        let content = self.pages[self.current].view(&self.card_data, &self.search_term);

        // Group all the elements together into a container sized to fill the whole window
        container(row!(controls_column, content))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // Use the dark theme
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

/// Enum used to store all the different application commands and user input data
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    /// Activates the scan card function manually, called when 'Scan Card' is clicked
    ScanCard,
    /// Exit the application, called when 'Exit' is clicked
    Exit,
    /// Attempt at a fullscreen button and to fix the issue with the app's resolution while on the desktop and in game mode
    Fullscreen,
    /// Sends the current search bar text to the app
    SearchInput(String),
    /// Sets the Settings page as the currently displayed page
    Settings,
    /// Sets the List page as the currently displayed page
    Home,
    /// (name, uuid) Changes the name for the card with the matching uuid
    ChangeCardName(String, String),
}
