use crate::app::theming;
use crate::app::Message;
use crate::scanning::{save_data_to_json, Card, Game, OtherLibrary};
use iced::widget::{button, column, container, scrollable, text, Column, Scrollable};
use iced::{Element, Length};

/// Returns a copy of the passed in list after it's been filtered by the search term
pub fn filter_list(list: &Vec<Card>, search_term: &str) -> Vec<Card> {
    list.into_iter()
        .map(|card| {
            // Use map to change the contents of each SD card's list by filter by the given search term
            Card {
                games: card
                    .games
                    .clone()
                    .into_iter()
                    .filter(|game| {
                        game.name
                            .to_ascii_lowercase()
                            .contains(&search_term.to_ascii_lowercase())
                    })
                    .collect(),
                name: card.name.clone(),
                uuid: card.uuid.clone(),
                heroic: if let Some(heroic) = card.heroic.clone() {
                    Some(OtherLibrary {
                        games: heroic
                            .games
                            .into_iter()
                            .filter(|game| {
                                game.name
                                    .to_ascii_lowercase()
                                    .contains(&search_term.to_ascii_lowercase())
                            })
                            .collect(),
                        path: heroic.path.clone(),
                    })
                } else {
                    None
                },
                lutris: if let Some(lutris) = card.lutris.clone() {
                    Some(OtherLibrary {
                        games: lutris
                            .games
                            .into_iter()
                            .filter(|game| {
                                game.name
                                    .to_ascii_lowercase()
                                    .contains(&search_term.to_ascii_lowercase())
                            })
                            .collect(),
                        path: lutris.path.clone(),
                    })
                } else {
                    None
                },
            }
        })
        .collect()
}

/// Returns the name of the currently inserted card, returns None if there is no inserted card or there was an issue getting the name.
pub fn get_card_name(list: &Vec<Card>) -> Option<String> {
    let current_uuid = crate::scanning::get_uuid()?;
    list.iter()
        .filter(|&card| card.uuid == current_uuid)
        // Filter for the card in the list with the same uuid as the inserted card
        .map(|card| card.name.clone())
        .next()
    // Get the next item which should be the one inserted card
}

/// Converts the list data into an Iced GUI list of the cards and their games
/// Also provides the search functionality by filtering the list data by the `search_term`
/// The `search_term` is is provided by the user in search bar
pub fn create_card_and_games_list<'a>(
    list: &'a Vec<Card>,
    search_term: &'a str,
) -> Scrollable<'a, Message> {
    let mut return_list: Vec<Element<Message>> = vec![];

    let list = if search_term.is_empty() {
        // If the search term is empty, don't filter the list
        list.clone()
    } else {
        filter_list(list, search_term)
    };

    for card in list {
        // List is seperated by different SD cards

        // If the lists of games on the card are empty and the search_term isn't empty
        // We'll assume the card has been 'filtered out' and wont display it at all
        if card.games.is_empty()
            && match &card.heroic {
                None => true,
                Some(heroic_list) => heroic_list.games.is_empty(),
            }
            && match &card.lutris {
                None => true,
                Some(lutris_list) => lutris_list.games.is_empty(),
            }
            && !search_term.is_empty()
        {
            continue;
        }

        return_list.push(text(format!("Card: {}", card.name)).size(50).into());

        if !card.games.is_empty() {
            if card.heroic.is_some() || card.lutris.is_some() {
                // Label the Steam Library if there's also Non Steam Libraries
                return_list.push(
                    container(
                        text(format!("Steam Games"))
                            .style(theming::STEAM_COLOR)
                            .size(40),
                    )
                    .into(),
                );
                return_list.push(
                    container(text(""))
                        .width(Length::Units(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Units(theming::DIVIDER_BAR_HEIGHT))
                        .padding(4)
                        .style(theming::STEAM_CONTAINER_STYLE)
                        .into(),
                );
            }

            return_list.push(
                //
                card.games
                    .iter()
                    .fold(column![], |column: Column<Message>, game: &Game| {
                        column.push(text(format!("{}", game.name)).size(30))
                    })
                    .into(),
            );
        }

        if let Some(library) = card.lutris {
            if !library.games.is_empty() {
                // If there were no games found or they were all filtered out by the search,
                // don't add any elements to differentiate the other libraries
                return_list.push(
                    text(format!("Lutris Library"))
                        .style(theming::LUTRIS_COLOR)
                        .size(40)
                        .into(),
                );

                // Add a divider line under the Lutris Library label
                return_list.push(
                    container(text(""))
                        .width(Length::Units(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Units(theming::DIVIDER_BAR_HEIGHT))
                        .padding(4)
                        .style(theming::LUTRIS_CONTAINER_STYLE)
                        .into(),
                );
                return_list.push(
                    library
                        .games
                        .iter()
                        .fold(column![], |column: Column<Message>, game: &Game| {
                            column.push(text(format!("{}", game.name)).size(30))
                        })
                        .into(),
                );
            }
        }

        if let Some(library) = card.heroic {
            if !library.games.is_empty() {
                return_list.push(
                    text(format!("Heroic Library"))
                        .style(theming::HEROIC_COLOR)
                        .size(40)
                        .into(),
                );
                return_list.push(
                    container(text(""))
                        .width(Length::Units(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Units(theming::DIVIDER_BAR_HEIGHT))
                        .padding(4)
                        .style(theming::HEROIC_CONTAINER_STYLE)
                        .into(),
                );
                return_list.push(
                    library
                        .games
                        .iter()
                        .fold(column![], |column: Column<Message>, game: &Game| {
                            column.push(text(format!("{}", game.name)).size(30))
                        })
                        .into(),
                );
            }
        }
    }

    scrollable(column(return_list).width(Length::Fill))
}

/// Gets the saved data from the json file in ~/.config and updates it with the currently inserted card
pub fn get_card_data() -> Vec<Card> {
    let list: Vec<Card> = match crate::scanning::get_saved_data() {
        Some(mut list) => {
            // Update the current cards data to the list, update the file
            crate::scanning::add_current_card(&mut list);
            crate::scanning::save_data_to_json(&list);
            list
        }
        None => match crate::scanning::scan_card(None) {
            // If the save file wasn't found, scan the current card if it exists and create the save file
            Some(card) => {
                crate::scanning::save_data_to_json(&vec![card.clone()]);
                vec![card]
            }
            None => {
                eprintln!("Error scanning SD card data");
                // If there wasn't any saved data found in ~/.config/sdscannerssave.json and no current SD card, return an empty list
                vec![]
            }
        },
    };

    list
}

pub fn control_button(label: &str, message: Message) -> Element<Message> {
    container(button(text(label).size(33)).padding(12).on_press(message))
        .padding(4)
        .into()
}

pub fn change_card_name(card_name: String, card_uuid: String, cards: &Vec<Card>) -> Vec<Card> {
    let mut return_list: Vec<Card> = vec![];

    for card in cards {
        // For all the cards in the passed in saved list -
        if card.uuid == card_uuid {
            // Check if the current card matches the uuid of the card who's name were trying to change
            // Push a clone card but with the name changed to the new passed in name
            return_list.push(Card {
                uuid: card.uuid.clone(),
                name: card_name.clone(),
                games: card.games.clone(),
                heroic: card.heroic.clone(),
                lutris: card.lutris.clone(),
            })
        } else {
            // Otherwise push the original card to the return list
            return_list.push(card.clone())
        }
    }

    save_data_to_json(&return_list);

    return_list
}

pub fn settings_label<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(5)
        .style(theming::SETTINGS_LABEL_CONTAINER_STYLE)
        .width(Length::Units(90))
        .height(Length::Units(60))
        .into()
}

pub fn long_settings_label<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(5)
        .style(theming::SETTINGS_LABEL_CONTAINER_STYLE)
        .width(Length::Units(310))
        .height(Length::Units(60))
        .into()
}

pub fn card_games_count(card: &Card) -> usize {
    let mut count = card.games.len();

    if let Some(heroic) = &card.heroic {
        count += heroic.games.len();
    }

    if let Some(lutris) = &card.lutris {
        count += lutris.games.len();
    }

    count
}
