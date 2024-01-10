use iced::{font::Family, window, Application, Font, Settings};

mod app;
mod scanning;

use app::App;

fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: (1280, 800),
            ..window::Settings::default()
        },
        default_font: Font {
            family: Family::Name("DejaVu Sans"), // The default sans serif font had rendering issues on the Steam Deck
            ..Default::default()
        },
        ..Settings::default()
    })
}
