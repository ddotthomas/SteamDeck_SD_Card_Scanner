use iced::theme::Theme;
use iced::widget::container;
use iced::{Color, Border};

// steam green 4d5a45 - ugly
// Color {
//     a: 1.0,
//     r: 0x4d as f32 / 255.0,
//     g: 0x5a as f32 / 255.0,
//     b: 0x45 as f32 / 255.0,
// };
// steam purple #BF00B4

pub static DIVIDER_BAR_HEIGHT: f32 = 6.0;
pub static DIVIDER_BAR_LENGTH: f32 = 450.0;

pub static STEAM_COLOR: Color = Color {
    a: 1.0,
    r: 0xBF as f32 / 255.0,
    g: 0x00 as f32 / 255.0,
    b: 0xB4 as f32 / 255.0,
};

// lutris r 247 g 153 b 79

pub static LUTRIS_COLOR: Color = Color {
    a: 1.0,
    r: 247.0 / 255.0,
    g: 153.0 / 255.0,
    b: 79.0 / 255.0,
};

// heroic 18dced

pub static HEROIC_COLOR: Color = Color {
    a: 1.0,
    r: 0x18 as f32 / 255.0,
    g: 0xdc as f32 / 255.0,
    b: 0xed as f32 / 255.0,
};

pub static LABEL_BORDER_COLOR: Color = Color {
    a: 1.0,
    r: 0x9c as f32 / 255.0,
    g: 0x9c as f32 / 255.0,
    b: 0x9c as f32 / 255.0,
};

/// Returns a container::Appearance to be used with .style(), sets the background color to the LUTRIS COLOR
pub static LUTRIS_CONTAINER_STYLE: fn(&Theme) -> container::Appearance =
    |_theme| container::Appearance {
        background: Some(LUTRIS_COLOR.into()),
        ..Default::default()
    };

/// Returns a container::Appearance to be used with .style(), sets the background color to the HEROIC COLOR
pub static HEROIC_CONTAINER_STYLE: fn(&Theme) -> container::Appearance =
    |_theme| container::Appearance {
        background: Some(HEROIC_COLOR.into()),
        ..Default::default()
    };

/// Returns a container::Appearance to be used with .style(), sets the background color to the STEAM COLOR
pub static STEAM_CONTAINER_STYLE: fn(&Theme) -> container::Appearance =
    |_theme| container::Appearance {
        background: Some(STEAM_COLOR.into()),
        ..Default::default()
    };

/// Returns a container::Appearance to be used with .style(), gives the container a border
pub static SETTINGS_LABEL_CONTAINER_STYLE: fn(&Theme) -> container::Appearance =
    |_theme| container::Appearance {
        border: Border { color: LABEL_BORDER_COLOR, width: 1.0, radius: 1.into() },
        ..Default::default()
    };

// I would like to organize the different styles and themes into an enum to help express the structure
// Can't get the different impl and attempts to work

// pub enum ContainerStyle {
//     SteamContainer,
//     LutrisContainer,
//     HeroicContainer,
// }
//
// impl From<ContainerStyle> for fn(&Theme) -> container::Appearance {
//     fn from(style: ContainerStyle) -> Self {
//         match style {
//             ContainerStyle::SteamContainer => |_theme| {
//                 container::Appearance {
//                     background: LUTRIS_COLOR.into(),
//                     ..Default::default()
//                 }
//             },
//             ContainerStyle::LutrisContainer => |_theme| {
//                 container::Appearance {
//                     background: LUTRIS_COLOR.into(),
//                     ..Default::default()
//                 }
//             },
//             ContainerStyle::HeroicContainer => |_theme| {
//                 container::Appearance {
//                     background: HEROIC_COLOR.into(),
//                     ..Default::default()
//                 }
//             }
//         }
//     }
// }

// impl container::StyleSheet for ContainerStyle {
//     type Style = Container;
//
//     fn appearance(&self, style: &Self::Style) -> container::Appearance {
//         match self {
//             ContainerStyle::SteamContainer => container::Appearance {
//                 background: STEAM_COLOR.into(),
//                 ..Default::default()
//             },
//             ContainerStyle::LutrisContainer => container::Appearance {
//                 background: LUTRIS_COLOR.into(),
//                 ..Default::default()
//             },
//             ContainerStyle::HeroicContainer => container::Appearance {
//                 background: HEROIC_COLOR.into(),
//                 ..Default::default()
//             },
//         }
//
//     }
// }

// pub fn return_style(&self) -> container::Appearance {
//     match self {
//         Self::SteamContainer => container::Appearance { background: STEAM_COLOR.into(), ..Default::default() },
//         Self::LutrisContainer =>
//         Self::HeroicContainer => container::Appearance { background: HEROIC_COLOR.into(), ..Default::default() }
//     }
// }
