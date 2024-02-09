use crate::app::utils;
use crate::app::utils::{long_settings_label, settings_label};
use crate::app::Message;
use crate::scanning::Card;
use crate::App;
use iced::widget::{column, container, row, text, text_input, Column};
use iced::{Element, Length};

pub enum Page {
    List,
    Settings,
}

impl<'a> Page {
    pub fn view(&'a self, app_data: &'a App) -> Element<Message> {
        match self {
            Page::List => Self::list(app_data).into(),
            Page::Settings => Self::settings(&app_data.card_data).into(),
        }
    }

    fn list(app_data: &'a App) -> Column<'a, Message> {
        let mut element_list: Vec<Element<Message>> = vec![container(row(vec![
            text_input("Filter Search...", &app_data.search_term)
                .on_input(|text_value| Message::SearchInput(text_value))
                .size(30)
                .width(Length::FillPortion(2))
                .padding(4)
                .into(),
            container(
                text(format!(
                    "Current Card: {}",
                    if let Some(card_name) = utils::get_card_name(&app_data.card_data) {
                        card_name
                    } else {
                        String::from("No Card Detected")
                    }
                ))
                .size(30),
            )
            .padding(2)
            .width(Length::FillPortion(3))
            .into(),
        ]))
        .padding(4)
        .into()];

        element_list.push(
            utils::create_card_and_games_list(app_data).into(),
        );

        utils::highlight_selection(&mut element_list, app_data.select_coords);

        column(element_list).width(Length::Fill)
    }

    // TODO
    fn settings(card_data: &Vec<Card>) -> Column<Message> {
        let mut element_list: Vec<Element<Message>> = vec![
            container(text("Settings - Work in Progress").size(40))
                .padding(2)
                .into(),
            row(vec![
                long_settings_label(text("Card Name").size(25)),
                settings_label(text("ID").size(25)),
                settings_label(text("Game(s)").size(25)),
            ])
            .into(),
        ];

        for card in card_data {
            let card_settings =
                row(vec![
                    long_settings_label(text_input(&card.name, &card.name).on_input(
                        |user_input| Message::ChangeCardName(user_input, card.uuid.clone()),
                    )),
                    settings_label(text(&card.uuid[..4])),
                    settings_label(text(format!("{}", utils::card_games_count(&card)))),
                ]);

            element_list.push(card_settings.into())
        }

        column(element_list)
    }
}
