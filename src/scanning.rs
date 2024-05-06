//! ## Scanning
//! 
//! The scanning module handles reading the game folders on the SD Card.

use crate::app::utils::is_sd_card_line;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    str,
};

/// Struct used to organize all the SD Cards data
#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    /// The UUID of the card, used to identify and save the cards info
    pub uuid: String,
    /// The SD card's name, can be set by the user. Otherwise, it is named '
    pub name: String,
    /// The list of Steam Games
    pub games: Vec<Game>,
    /// The Heroic Games library if one was found
    pub heroic: Option<OtherLibrary>,
    /// The Lutris library if one was found
    pub lutris: Option<OtherLibrary>,
}

/// Struct used to organize any data about a game found in a library folder
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    /// The game's folder name
    pub name: String,
}

/// Struct for game other game libraries that aren't Steam, Lutris and Heroic for now
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OtherLibrary {
    /// The path of the libraries folder, will be used to scan the already found folder and skip searching for it again.
    pub path: PathBuf,
    /// List of games found in the library's folder
    pub games: Vec<Game>,
}

/// Enum used to tell different functions what library to work on
#[derive(Copy, Clone)]
enum LibraryType {
    Lutris,
    Heroic,
    Other,
    Game,
}

pub struct ScanData {
    pub card_path: PathBuf,
    pub uuid: String,
    pub name: Option<String>,
}

/// Modifies the passed in list with the currently inserted SD cards game data
pub fn update_list(list: &mut Vec<Card>) {
    // Instead of checking if the card is on the list, just always update whatever info is at that UUID,
    // Now the function can update the games list while scanning

    let mut cards: HashMap<String, Card> = list.iter().fold(HashMap::new(), |mut map, card| {
        // label the list of cards by their uuid
        map.insert(card.uuid.clone(), card.clone());
        map
    });

    // Get a list of the inserted cards
    let cards_to_scan: Vec<ScanData> = if let Some(cards) = get_card_info() {
        cards
    } else {
        // If there was an issue getting the list of cards, return without modifying the list
        return;
    };

    // For each SD card found in the lsblk scan
    for card_to_scan in cards_to_scan {
        // Check to see if this card was scanned before and is already on the saved list
        match cards.get_mut(&card_to_scan.uuid) {
            // get a mutable reference (card) to the currently inserted SD card list item
            Some(card) => {

                // Attempt to scan new card data, if it's successful, update the card with the new scanned info
                if let Some(scanned_card) = scan_card(ScanData {
                    name: Some(card.name.clone()),
                    ..card_to_scan
                }) {
                    *card = scanned_card
                }
            }
            // If the current card isn't in the list, get its data and add it to the HashMap of cards
            None => {
                let scanned_card = match scan_card(card_to_scan) {
                    Some(scanned_card) => scanned_card,
                    None => {
                        eprintln!("Couldn't scan card after finding the UUID for it");
                        continue;
                    }
                };

                cards.insert(scanned_card.uuid.clone(), scanned_card);
            }
        }
    }
    *list = cards.values().fold(vec![], |mut vec, entry| {
        // Move the values from the hashmap back into the list
        vec.push(entry.clone());
        vec
    });
}

/// Get the data for the current card
pub fn scan_card(data: ScanData) -> Option<Card> {
    let name = if let Some(name) = data.name {
        name
    } else {
        String::from("SD Card 1")
    };

    let mut steam_dir = data.card_path.clone();
    steam_dir.push("steamapps/common");

    let games = find_games(&steam_dir)?;

    let (lutris, heroic) = find_other_game_folders(&data.card_path);

    let card = Card {
        // Collect all the data for the card before returning it from the function
        uuid: data.uuid,
        name,
        games,
        lutris,
        heroic,
    };

    Some(card)
}

/// Runs the `lsblk` command on the system to get the UUID and Path for the inserted SD Cards
pub fn get_card_info() -> Option<Vec<ScanData>> {
    // Run the lsblk command to get the currently inserted SD cards
    let output = match Command::new("lsblk")
        .arg("-o")
        .arg("NAME,UUID,MOUNTPOINT")
        .output()
    {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Error running lsblk command: {}", e);
            return None;
        }
    };

    let stdout = output.stdout;
    // Turn the command output into a string
    let s = match str::from_utf8(&stdout) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to convert output to string: {}", e);
            return None;
        }
    }
    .to_string();

    let card_lines: Vec<String> = s // Find all lines that mention an SD card to be scanned
        .lines()
        .filter(|line| is_sd_card_line(*line))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut scan_data_list: Vec<ScanData> = vec![];

    for card_line in card_lines {
        let mut word_iter = card_line.split_whitespace();

        let uuid: String = match word_iter.nth(1) {
            // Split the string by whitespace and get the second word, the UUID
            Some(uuid) => uuid,
            None => {
                println!("Couldn't find UUID from lsblk command");
                return None;
            }
        }
        .to_string();

        let card_path: PathBuf = match word_iter.map(|path| PathBuf::from(path)).next() {
            Some(path) => path,
            None => continue,
        };

        scan_data_list.push(ScanData {
            uuid,
            card_path,
            name: None,
        })
    }

    Some(scan_data_list)
}

/// Gets the saved data from the json file in ~/.config and updates it with the currently inserted cards
pub fn get_card_data() -> Vec<Card> {
    let list: Vec<Card> = match crate::scanning::get_saved_json_data() {
        Some(mut list) => {
            // Update the current cards data to the list; update the file
            crate::scanning::update_list(&mut list);
            crate::scanning::save_data_to_json(&list);
            list
        }
        None => {
            // If there wasn't a save file or it couldn't be read, create a new list of cards
            let list_of_cards = create_new_card_list();
            // Save the new list to the json save file
            crate::scanning::save_data_to_json(&list_of_cards);
            list_of_cards
        }
    };

    list
}

/// Scans the currently inserted cards and adds the data to a new empty list
fn create_new_card_list() -> Vec<Card> {
    let mut list_of_cards: Vec<Card> = vec![];
    let cards_to_scan = match get_card_info() {
        Some(scan_data_list) => scan_data_list,
        // None is only returned if there was an error or major issue
        // The list will be empty if there's no SD cards and no errors occurred.
        None => return list_of_cards,
    };
    for card_to_scan in cards_to_scan {
        match crate::scanning::scan_card(card_to_scan) {
            Some(card) => list_of_cards.push(card),
            None => {
                eprintln!("Error scanning SD card data");
            }
        }
    }

    list_of_cards
}

/// Scans ~/.config/sdscannersave.json, returns None if file doesn't exist or there's a problem parsing the json contents
pub fn get_saved_json_data() -> Option<Vec<Card>> {
    let config_loc: PathBuf = PathBuf::from(dirs::config_dir().unwrap()).join("sdscannersave.json");
    if !config_loc.is_file() {
        // if there's no file at the save path, just assume it doesn't exist and quietly return none
        return None;
    }

    let card_data: Vec<Card> =
        match serde_json::from_str(fs::read_to_string(config_loc).unwrap().as_str()) {
            Ok(json) => json,
            Err(e) => {
                // Print an error if the contents were modified so that they don't match the Vec<Card> type
                eprintln!("Problem parsing json file ~/.config/sdscannersave.json\n{e}");
                return None;
            }
        };

    Some(card_data)
}

/// Scans the passed in folder for all the game's folders inside. Returns None if there was an error reading the game's directory
fn find_games(search_dir: &Path) -> Option<Vec<Game>> {
    let mut list: Vec<Game> = match fs::read_dir(search_dir) {
        Ok(entry) => entry,
        Err(e) => {
            eprintln!("Couldn't read game directory: {}", e);
            return None;
        }
    }
    .map(|entry| entry.unwrap())
    .map(|item| item.path())
    .filter(|path| path.is_dir()) // Make sure the item in the folder is directory, removes things like dll files
    .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
    .map(|file_name| Game { name: file_name }) // Get the folders name for the Game data, add that data to a list
    .fold(vec![], |mut vec, entry| {
        vec.push(entry);
        vec
    });

    list.sort_by_key(|game| game.name.to_ascii_lowercase());

    Some(list)
}

/// Saves the list of Card data into a json file into the user's .config folder. (~/.config/sdscannersave.json)
pub fn save_data_to_json(list: &Vec<Card>) {
    let save_data_path: PathBuf =
        PathBuf::from(dirs::config_dir().unwrap()).join("sdscannersave.json");

    let s = serde_json::to_string(&list).unwrap();

    match fs::write(save_data_path, s) {
        Ok(()) => {}
        Err(e) => eprintln!("Couldn't save data to ~/.config/sdscannersave.json: {}", e),
    }
}

/// Scan for Lutris and Heroic libraries and return them as a tuple (lutris, heroic)
fn find_other_game_folders(search_dir: &Path) -> (Option<OtherLibrary>, Option<OtherLibrary>) {
    let lutris: Option<OtherLibrary> = search_and_scan_folder(search_dir, LibraryType::Lutris);
    let heroic: Option<OtherLibrary> = search_and_scan_folder(search_dir, LibraryType::Heroic);

    (lutris, heroic)
}

// TODO restructure this code to recursively call a search instead of the weird match logic currently
/// Scan for other game folders, uses LibraryType enum as a switch to check for different types of libraries (Lutris or Heroic)
fn search_and_scan_folder(card_path: &Path, library_type: LibraryType) -> Option<OtherLibrary> {
    let mut library = OtherLibrary::default();

    // Search for the library type in the card's root first,
    match scan_folder_for_library(card_path, library_type) {
        // If there's 1 found folder for the library type, assume it's the correct one
        Some(dirs) if dirs.len() == 1 => {
            library.path = dirs[0].clone();
        },
        // If the searched for library wasn't found, search for the 'Other' library type
        Some(dirs) if dirs.len() == 0 => {
            match scan_folder_for_library(card_path, LibraryType::Other) {
                Some(dirs) if dirs.len() >= 1 => {
                    // Now, if we found an 'Other' folder, search it for the library type we're originally searching for
                    for dir in dirs {
                        match scan_folder_for_library(&dir, library_type) {
                            Some(dirs) if dirs.len() == 1 => {
                                library.path = dirs[0].clone();
                                break;
                            }
                            Some(dirs) if dirs.len() == 0 => continue,
                            // In any case where lutris or heroic returns several folders, not sure what a good solution would be
                            Some(_) => {}
                            None => return None,
                        }
                    }
                }
                // Right now I'm only searching for folders that have the word "games" or "other" for a lutris or heroic folder, this happens if lutris and heroic weren't found at the SD card's root which I imagine should be the common configuration
                Some(dirs) if dirs.len() == 0 => {
                    match scan_folder_for_library(card_path, LibraryType::Game) {
                        Some(dirs) if dirs.len() >= 1 => {
                            for dir in dirs {
                                match scan_folder_for_library(&dir, library_type) {
                                    Some(dirs) if dirs.len() == 1 => {
                                        library.path = dirs[0].clone();
                                        break;
                                    }
                                    Some(dirs) if dirs.len() == 0 => continue,
                                    // Again, in any case where lutris or heroic returns several folders, not sure what a good solution would be
                                    Some(_) => {}
                                    None => return None,
                                }
                            }
                        }
                        Some(_) => {}
                        None => return None,
                    }
                }
                Some(_) => {}
                // None(s) are only called when read_dir() encounters an error, we'll assume some sort of IO or file perm issue, doesn't matter
                None => return None,
            }
        },
        // Again, not sure how I'll handle situations where more than one folder is found
        Some(_) => {}
        None => return None,
    };

    // Check if there was any folder found for the library, if there wasn't exit the function
    if library.path == PathBuf::new() {
        return None;
    }

    library.games = find_games(&library.path)?;

    Some(library)
}

/// Scans passed in dir based on passed in LibraryType returning Some(Vec<PathBuf>) with the vec being empty if no folders were found, None is returned in cases of an error
fn scan_folder_for_library(dir: &Path, t: LibraryType) -> Option<Vec<PathBuf>> {
    Some(
        // Attempt to read directory, fails for permission issues or if path isn't a dir
        match fs::read_dir(dir) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Couldn't scan SD card for other game libraries\nError: {e}");
                return None;
            }
        }
        // Convert each element into a path
        .map(|entry| entry.unwrap().path())
        // Filter for all the paths that are a directory and are labeled with one of the library names
        .filter(|path| path.is_dir())
        .filter(|dir| {
            dir.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_ascii_lowercase()
                .contains(match t {
                    LibraryType::Lutris => "lutris",
                    LibraryType::Heroic => "heroic",
                    LibraryType::Other => "other",
                    LibraryType::Game => "game",
                })
        })
        // Collect all the found directories into a Vec
        .collect(),
    )
}
