use iced::{window, Application, Settings, Size};

mod app;
mod scanning;

use app::App;

fn main() -> iced::Result {
    // Run our Iced app with some modified application settings
    App::run(Settings {
        window: window::Settings {
            // Set the window to the size of the Steam Deck
            size: Size::new(1280.0, 800.0),
            // Tried to use max_size at one point to fix the mouse cursor not reaching the bottom left
            // max_size: Some((1280, 800)),
            // Set the rest of the window settings to the default
            ..window::Settings::default()
        },
        // Set the rest of the application settings to the default
        ..Settings::default()
    })
}
