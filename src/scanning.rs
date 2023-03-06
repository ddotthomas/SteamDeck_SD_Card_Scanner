use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    str,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub uuid: String,
    pub name: String,
    pub games: Vec<Game>,
    pub heroic: Option<OtherLibrary>,
    pub lutris: Option<OtherLibrary>,
}

// new idea again, have lutris and heroic be an Option Vec<Game> to then both label and display on the list, can have nice H3 headers labeling the different libraries as steam or otherwise.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OtherLibrary {
    path: PathBuf,
    pub games: Vec<Game>,
}

#[derive(Copy, Clone)]
enum LibraryType {
    Lutris,
    Heroic,
    Other,
    Game,
}

pub const STEAM_DIR: &'static str = "/run/media/mmcblk0p1/steamapps/common";
pub const SD_DIR: &'static str = "/run/media/mmcblk0p1";

/// Function updates the currently inserted SD card game data to the passed in list
pub fn add_current_card(list: &mut Vec<Card>) {
    //   Instead of checking if the card is on the list, just always update whatever info is at that UUID,
    // Now the function can update the games list and do two things at once
    let current_card_uuid = match get_uuid() {
        Some(uuid) => uuid,
        None => {
            println!("Couldn't get card's UUID, is one inserted?");
            return;
        }
    };
    let mut cards: HashMap<String, Card> = list.iter().fold(HashMap::new(), |mut map, card| {
        // label the list of cards by their uuid
        map.insert(card.uuid.clone(), card.clone());
        map
    });

    match cards.get_mut(&current_card_uuid) {
        // get a mutable reference to the card currently in the list
        Some(card) => {
            *card = match scan_card(Some(list)) {
                // If we find a card, update its data with the new scanned data
                Some(scanned_card) => scanned_card,
                None => card.clone(), // if for some reason we could get the cards uuid but now cant scan new info from it,
            }
        }
        None => {
            // If the current card isn't in the list, get its data and add it to the HashMap of cards
            let scanned_card = match scan_card(Some(list)) {
                Some(scanned_card) => scanned_card,
                None => {
                    eprintln!("Couldn't scan card after finding the UUID for it");
                    return;
                }
            };

            cards.insert(scanned_card.uuid.clone(), scanned_card);
        }
    }

    *list = cards.values().fold(vec![], |mut vec, entry| {
        // Move the values from the hashmap back into the list
        vec.push(entry.clone());
        vec
    });
}

/// Get the data for the current card, the card's name gets decided from the passed in list
pub fn scan_card(list: Option<&Vec<Card>>) -> Option<Card> {
    let uuid = match get_uuid() {
        Some(uuid) => uuid,
        None => {
            println!("Couldn't get card's UUID, make sure its inserted");
            return None;
        }
    };

    let name = match list {
        Some(list) => {
            let saved_card: Vec<Card> =
                list.iter()
                    .filter(|card| *card.uuid == uuid)
                    .fold(vec![], |mut vec, card| {
                        vec.push(card.clone());
                        vec
                    });

            if saved_card.len() == 1 {
                saved_card[0].name.clone()
            }
            // If the card was saved before, grab its saved name
            else {
                format!("SD Card {}", list.len() + 1)
            } // otherwise add 1 number to the amount of current cards
        }

        None => String::from("SD Card 1"),
    };

    println!("Looking for games at {}", STEAM_DIR);
    let games = find_games(&PathBuf::from(STEAM_DIR))?;

    let (lutris, heroic) = find_other_game_folders();

    let card = Card {
        uuid,
        name,
        games,
        lutris,
        heroic,
    };

    Some(card)
}

pub fn get_uuid() -> Option<String> {
    let output = match Command::new("lsblk")
        .arg("-o")
        .arg("MOUNTPOINT,UUID")
        .output()
    {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Error running lsblk command: {}", e);
            return None;
        }
    };

    let stdout = output.stdout;

    let s = match str::from_utf8(&stdout) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to convert output to string: {}", e);
            return None;
        }
    }
    .to_string();

    let card_line: String = s // Find the line of output that has the SD Card
        .lines()
        .filter(|line| line.contains("/run/media/mmcblk0p1"))
        .collect::<String>();

    let uuid: String = match card_line // Split the string by whitespace and get the second word, the UUID
        .split_whitespace()
        .nth(1)
    {
        Some(uuid) => uuid,
        None => {
            println!("Couldn't find UUID from lsblk command");
            return None;
        }
    }
    .to_string();

    Some(uuid)
}

/// Scans ~/.config/sdscannersave.json, returns None if file doesn't exist or there's a problem parsing the json contents
pub fn get_saved_data() -> Option<Vec<Card>> {
    let config_loc: PathBuf = PathBuf::from(dirs::config_dir().unwrap()).join("sdscannersave.json");
    if !config_loc.is_file() {
        // if there's no file at the save path, just assume it doesn't exist and quietly return none
        return None;
    }

    let card_data: Vec<Card> =
        match serde_json::from_str(fs::read_to_string(config_loc).unwrap().as_str()) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Problem parsing json file ~/.config/sdscannersave.json\n{e}"); // Print an error if the contents were modified so that they don't match the Vec<Card> type
                return None;
            }
        };

    Some(card_data)
}

/// Scan the passed in folder for all folders it contains which have their file name saved into a Game struct; returns None if there was an error reading the game's directory
fn find_games(search_dir: &Path) -> Option<Vec<Game>> {
    let list: Vec<Game> = match fs::read_dir(search_dir) {
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
    .map(|file| Game { name: file }) // Get the folders name for the Game data, add that data to a list
    .fold(vec![], |mut vec, entry| {
        vec.push(entry);
        vec
    });

    Some(list)
}

pub fn save_data(list: &Vec<Card>) {
    let save_data_path: PathBuf =
        PathBuf::from(dirs::config_dir().unwrap()).join("sdscannersave.json");

    let s = serde_json::to_string(&list).unwrap();

    match fs::write(save_data_path, s) {
        Ok(()) => {}
        Err(e) => eprintln!("Couldn't save data to ~/.config/sdscannersave.json: {}", e),
    }
}

/// Scan for Lutris and Heroic libraries and return them as a hashmap (k: PathBuf,v: Vec<Game>)
fn find_other_game_folders() -> (Option<OtherLibrary>, Option<OtherLibrary>) {
    let lutris: Option<OtherLibrary> = search_and_scan_folder(LibraryType::Lutris);
    let heroic: Option<OtherLibrary> = search_and_scan_folder(LibraryType::Heroic);

    (lutris, heroic)
}

/// Scan for other game folders, uses LibraryType enum as a switch to check for different types of libraries (Lutris or Heroic)
fn search_and_scan_folder(t: LibraryType) -> Option<OtherLibrary> {
    let mut library = OtherLibrary::default();

    match scan_folder_for_library(SD_DIR, t) {
        Some(dirs) if dirs.len() == 1 => {
            library.path = dirs[0].clone();
        }

        Some(dirs) if dirs.len() == 0 => {
            match scan_folder_for_library(SD_DIR, LibraryType::Other) {
                Some(dirs) if dirs.len() >= 1 => {
                    for dir in dirs {
                        match scan_folder_for_library(dir.to_str().unwrap(), t) {
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
                    match scan_folder_for_library(SD_DIR, LibraryType::Game) {
                        Some(dirs) if dirs.len() >= 1 => {
                            for dir in dirs {
                                match scan_folder_for_library(dir.to_str().unwrap(), t) {
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
        }
        // Again, not sure how I'll handle situations where more than one folder is found
        Some(_) => {}
        None => return None,
    };

    // Check if there was any folder found for the library, if there wasn't exit the function
    if library.path == PathBuf::new() {
        return None;
    }

    println!("Looking for games at {}", library.path.to_string_lossy());
    let path_fix = library.path.to_str().unwrap();
    library.games = find_games(&PathBuf::from(path_fix))?;

    println!("Found other game: {:?}", library.games);

    Some(library)
}

/// Scans passed in dir based on passed in LibraryType returning Some(Vec<PathBuf>) with the vec being empty if no folders were found, None is returned in cases of an error
fn scan_folder_for_library(dir: &str, t: LibraryType) -> Option<Vec<PathBuf>> {
    Some(
        match fs::read_dir(dir) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Couldn't scan SD card for other game libraries\nError: {e}");
                return None;
            }
        }
        .map(|entry| entry.unwrap())
        .map(|dir| dir.path())
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
        .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::scanning::*;

    #[test]
    fn test_get_uuid() {
        let result = get_uuid().unwrap();

        //let s: String = result.lines().filter(|line| line.contains("media/")).collect();

        println!("Got result: {}", result);
    }

    #[test]
    fn test_find_games() {
        let result = find_games(&PathBuf::from(STEAM_DIR)).unwrap();

        println!("Got results: {:?}", result);
    }
}
