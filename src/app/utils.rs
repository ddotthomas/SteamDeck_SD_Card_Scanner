//! ## Utils
//! 
//! Holds a bunch of miscellanious utility functions used in different parts of the function

use crate::app::theming;
use crate::app::Message;
use crate::scanning::{self, save_data_to_json, Card, Game, OtherLibrary};
use iced::widget::{button, column, container, scrollable, text, Column};
use iced::{Element, Length};

/// Returns a copy of the passed in list after it's been filtered by the search term
pub fn filter_list(list: &Vec<Card>, search_term: &str) -> Vec<Card> {
    list.into_iter()
        // Use map to change the contents of each SD card's list by filter by the given search term
        .map(|card| {
            // Create a copy card
            Card {
                // filter the games based on if they match the search_term
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
                // Check if there's a list of heroic library games, and then filter it
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
                // Also check for and filter the lutris list
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
        // Collect the Card clones into a Vec to return
        .collect()
}

// TODO needs to return possibly multiple names for multiple inserted cards
/// Returns the name of the currently inserted card, returns None if there is no inserted card or there was an issue getting the name.
pub fn get_card_name(list: &Vec<Card>) -> Option<String> {
    // Scan for the cards in the system, getting their UUID
    let mut inserted_cards = scanning::get_card_info()?;

    // If no cards were scanned, then return None
    if inserted_cards.len() == 0 {
        return None;
    }

    for scanned_card in &mut inserted_cards {
        scanned_card.name = list
            .iter()
            // Filter the saved card data for a card with a matching UUID as the currently inserted card
            .filter(|&card| card.uuid == scanned_card.uuid)
            // Grab that saved cards name
            .map(|card| card.name.clone())
            .next();
    }

    // This needs some work, to handle multiple inserted cards
    if inserted_cards.len() >= 1 {
        inserted_cards[0].name.clone()
    } else {
        None
    }
}

/// Converts the list data into an Iced GUI list of the cards and their games
/// Also provides the search functionality by filtering the list data by the `search_term`
/// The `search_term` is is provided by the user in search bar
pub fn create_card_and_games_list<'a>(
    list: &'a Vec<Card>,
    search_term: &'a str,
) -> Element<'a, Message> {
    let mut return_list: Vec<Element<Message>> = vec![];

    let list = if search_term.is_empty() {
        // If the search term is empty, don't filter the list

        // TODO rework the list to be: mut Vec<& Card>
        // allow us to modify the list directy and avoid the clone
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
                // Add a divider bar
                return_list.push(
                    container(text(""))
                        .width(Length::Fixed(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Fixed(theming::DIVIDER_BAR_HEIGHT))
                        .padding(4)
                        .style(theming::STEAM_CONTAINER_STYLE)
                        .into(),
                );
            }

            // Collect the Steam Games into a column 
            return_list.push(
                card.games
                    .iter()
                    .fold(column![], |column: Column<Message>, game: &Game| {
                        column.push(text(format!("{}", game.name)).size(30))
                    })
                    .into(),
            );
        }

        // If a lutris library was detected on the card, add its games to the list
        if let Some(library) = card.lutris {
            // If there were no games found or they were all filtered out by the search,
            // don't add any of the labeling and dividers
            if !library.games.is_empty() {
                // Add a label to this list of games
                return_list.push(
                    text(format!("Lutris Library"))
                        .style(theming::LUTRIS_COLOR)
                        .size(40)
                        .into(),
                );

                // Add a colored divider line under the Lutris Library label
                return_list.push(
                    container(text(""))
                        .width(Length::Fixed(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Fixed(theming::DIVIDER_BAR_HEIGHT))
                        .padding(4)
                        .style(theming::LUTRIS_CONTAINER_STYLE)
                        .into(),
                );
                // Finally add the list of games
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

        // Do the same checks above for heroic
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
                        .width(Length::Fixed(theming::DIVIDER_BAR_LENGTH))
                        .height(Length::Fixed(theming::DIVIDER_BAR_HEIGHT))
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

    // Encase the parsed list in a scrollable container
    scrollable(column(return_list).width(Length::Fill)).into()
}

/// Creates a button that's themed to be on the control menu
pub fn control_button(label: &str, message: Message) -> Element<Message> {
    container(button(text(label).size(33)).padding(12).on_press(message))
        .padding(4)
        .into()
}

/// Changes the name of the card matching the passed in UUID. Writes the changes to json save file
pub fn change_card_name(card_name: String, card_uuid: String, cards: &Vec<Card>) -> Vec<Card> {
    let mut return_list: Vec<Card> = vec![];

    // For all the cards in the passed in saved list -
    for card in cards {
        // Check if the current card matches the uuid of the card who's name were trying to change
        if card.uuid == card_uuid {
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

    // Write the modified data to disk
    save_data_to_json(&return_list);

    // Return the modified list
    return_list
}

/// Applies theming to an iced Element for the Settings page
/// 
/// Gives all the different containers the same width and height so they arrange in a table
pub fn settings_label<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(5)
        .style(theming::SETTINGS_LABEL_CONTAINER_STYLE)
        .width(Length::Fixed(90.0))
        .height(Length::Fixed(60.0))
        .into()
}

/// Applies theming to a container for the Settings page
/// 
/// Applies the width and height to be in the table but much longer
pub fn long_settings_label<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(5)
        .style(theming::SETTINGS_LABEL_CONTAINER_STYLE)
        .width(Length::Fixed(310.0))
        .height(Length::Fixed(60.0))
        .into()
}

/// Counts the amount of games on the passed in Card
pub fn card_games_count(card: &Card) -> usize {
    // Start the count with the amount of Steam games
    let mut count = card.games.len();

    // Then depending if the heroic and lutris library were detected,
    // add their games to the count too.
    if let Some(heroic) = &card.heroic {
        count += heroic.games.len();
    }

    if let Some(lutris) = &card.lutris {
        count += lutris.games.len();
    }

    count
}

/// The default path to the SD card's root folder before v3.5 of SteamOS
pub const OLD_SD_ROOT: &'static str = "/run/media/mmcblk0p1";
/// The new mount folder for SD cards after v3.5 of SteamOS
pub const NEW_SD_PATH: &'static str = "/run/media/deck";

/// Checks if the string has info on the inserted SD card
pub fn is_sd_card_line(line: &str) -> bool {
    line.contains(OLD_SD_ROOT) | line.contains(NEW_SD_PATH) && line.contains("mmcblk")
}
