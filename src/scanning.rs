use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    str,
    collections::HashMap,
};


#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub uuid: String,
    pub name: String,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub name: String,
}

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
    let mut cards: HashMap<String, Card> = list.iter().fold(HashMap::new(), |mut map, card| {   // label the list of cards by their uuid
        map.insert(card.uuid.clone(), card.clone());
        map
    });

    match cards.get_mut(&current_card_uuid) {   // get a mutable reference to the card currently in the list
        Some(card) => { *card = match scan_card(Some(list)) {  // If we find a card, update its data with the new scanned data
            Some(scanned_card) => scanned_card,
            None => card.clone(),  // if for some reason we could get the cards uuid but now cant scan new info from it, 
        } },
        None => { // If the current card isn't in the list, get its data and add it to the HashMap of cards
            let scanned_card = match scan_card(Some(list)) {
                Some(scanned_card) => scanned_card,
                None => {eprintln!("Couldn't scan card after finding the UUID for it"); return; }
            };

            cards.insert(scanned_card.uuid.clone(), scanned_card);
        },
    }

    *list = cards.values().fold( vec![], |mut vec, entry| {  // Move the values from the hashmap back into the list
        vec.push(entry.clone());
        vec
    });

    save_data(list);
}


pub fn scan_card(list: Option<& Vec<Card>>) -> Option<Card> {
    // Get the data for the current card, optionally add the current list of card to give this card a name
    let uuid = match get_uuid() {
        Some(uuid) => uuid,
        None => { println!("Couldn't get card's UUID, make sure its inserted"); return None }
    };

    let card_num: u32 = match list {
        Some(list) => (list.len() + 1).try_into().unwrap(),
        None => 1,
    };

    let name = String::from(format!("SD Card {}", card_num));

    let games = find_games().unwrap();

    let card = Card { uuid, name, games };

    save_data(&vec![card.clone()]);

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

pub fn get_saved_data() -> Option<Vec<Card>> {
    let config_loc: PathBuf = PathBuf::from(dirs::config_dir().unwrap()).join("sdscannersave.json");
    if !config_loc.is_file() {
        return None;
    }

    let card_data: Vec<Card> =
        serde_json::from_str(fs::read_to_string(config_loc).unwrap().as_str()).unwrap();

    Some(card_data)
}

fn find_games() -> Option<Vec<Game>> {
    let list: Vec<Game> = match fs::read_dir(Path::new(  // Get an iterator over all the folders in the steam common folder
        "/run/media/mmcblk0p1/steamapps/common",
    )) {
        Ok(entry) => entry,
        Err(e) => {
            eprintln!("Couldn't read Steam directory: {}", e);
            return None;
        }
    }
    .map(|entry| entry.unwrap())  
    .map(|item| item.path())
    .filter(|path| path.is_dir())  // Make sure the item in the folder is directory, removes things like dll files
    .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
    .map(|file| Game { name: file })  // Get the folders name for the Game data, add that data to a list
    .fold(vec![], |mut vec, entry| {
        vec.push(entry);
        vec
    });

    Some(list)
}

fn save_data(list: & Vec<Card>) {
    let save_data_path: PathBuf = PathBuf::from(dirs::config_dir().unwrap()).join("sdscannersave.json");

    let s = serde_json::to_string(&list).unwrap();

    match fs::write(save_data_path, s) {
        Ok(()) => {},
        Err(e) => eprintln!("Couldn't save data to ~/.config/sdscannersave.json: {}", e),
    }
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
        let result = find_games().unwrap();

        println!("Got results: {:?}", result);
    }
}
