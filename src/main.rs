use iced::{font::Family, window, Application, Font, Settings};

mod app;
mod scanning;

use app::App;

fn main() -> iced::Result {
    let size = (1280, 800);
    App::run(Settings {
        window: window::Settings {
            size,
            // max_size: Some((1280, 800)),
            ..window::Settings::default()
        },
        default_font: Font {
            family: Family::Name("DejaVu Sans"),
            ..Default::default()
        },
        ..Settings::default()
    })
}
