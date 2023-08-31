use crate::scanning::{Card, OtherLibrary};

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
