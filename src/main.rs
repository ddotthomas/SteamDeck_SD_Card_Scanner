use iced::{window, Application, Settings, Size};

mod app;
mod scanning;

use app::App;

fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: Size::new(1280.0, 800.0),
            // max_size: Some((1280, 800)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
