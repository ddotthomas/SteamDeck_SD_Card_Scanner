use crate::app::utils;
use crate::app::utils::{long_settings_label, settings_label};
use crate::app::Message;
use crate::scanning::Card;
use iced::widget::{column, container, row, text, text_input, Column};
use iced::{Element, Length};

pub enum Page {
    List,
    Settings,
}

impl<'a> Page {
    /// view() probably shouldn't be designed to require card_data or search_term but it works for now
    pub fn view(&'a self, card_data: &'a Vec<Card>, search_term: &'a str) -> Element<Message> {
        match self {
            Page::List => Self::list(card_data, search_term).into(),
            Page::Settings => Self::settings(card_data).into(),
        }
    }

    fn list(list: &'a Vec<Card>, search_term: &'a str) -> Column<'a, Message> {
        let mut element_list: Vec<Element<Message>> = vec![container(row(vec![
            text_input("Filter Search...", search_term)
                .on_input(|text_value| Message::SearchInput(text_value))
                .size(30)
                .width(Length::FillPortion(2))
                .padding(4)
                .into(),
            container(
                text(format!(
                    "Current Card: {}",
                    if let Some(card_name) = utils::get_card_name(list) {
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

        element_list.push(utils::create_card_and_games_list(list, search_term).into());
        column(element_list).width(Length::Fill)
    }

    // TODO
    fn settings(list_data: &Vec<Card>) -> Column<Message> {
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

        for card in list_data {
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
