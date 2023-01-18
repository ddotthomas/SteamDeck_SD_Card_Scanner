use std::{fs::{self, DirEntry}, process::Command, str, path::Path, };
use serde_json::{self, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub uuid: String,
    pub name: String,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub name: String,
}

pub fn scan_card() -> Card {
    let uuid = get_uuid().unwrap();

    let name = String::from("Card 1");

    let games = find_games().unwrap();

    Card {
        uuid,
        name,
        games,
    }
}

fn get_uuid() -> Option<String> {
    let output = Command::new("lsblk")
        .arg("-o")
        .arg("MOUNTPOINT,UUID")
        .output()
        .expect("failed to run lsblk");

    let stdout = output.stdout;

    let s = match str::from_utf8(&stdout) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to convert output to string: {}", e);
            return None;
        }
    }
    .to_string();

    let card_line: String = s
        .lines()
        .filter(|line| line.contains("/run/media/mmcblk0p1"))
        .collect::<String>();

    let uuid: String = card_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string();

    Some(uuid)
}

fn find_games() -> Option<Vec<Game>> {
    let list: Vec<Game> = fs::read_dir(Path::new("/media/todd/Storage/SteamLibrary/steamapps/common"))
        .expect("Failed to read steam dir")
        .map(|entry| entry.unwrap())
        .map(|item| item.path())
        .filter(|path| path.is_dir())
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .map(|file| Game { name: file })
        .fold(vec![], |mut vec, entry| { vec.push(entry); vec});

    Some(list)
}

#[cfg(test)]
mod tests {
    use crate::scanner::*;

    #[test]
    fn test_get_uuid() {
        let result = get_uuid().unwrap();

        //let s: String = result.lines().filter(|line| line.contains("media/")).collect();

        println!("Got result: {}", result);
    }

    #[test]
    fn test_find_games() {
        let result =  find_games().unwrap();

        println!("Got results: {:?}", result);
    }
}
