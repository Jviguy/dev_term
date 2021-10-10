use strum::IntoEnumIterator;
use strum::EnumIter;

use std::collections::HashMap;

#[derive(EnumIter, PartialEq)]
enum CommandMap {
    // store commands here
}

impl CommandMap {

    pub fn closest_match(&self, target: &str) -> Option<String> { // todo: update for strum
        /*let mut matches: HashMap<u8, String> = HashMap::new();
        for (name, _) in self.commands.iter() {
            matches.insert(levenshtein(name.as_str(), target) as u8, name.to_string());
        }
        if matches.len() > 0 {
            return Some(matches[matches.keys().min().unwrap()].clone());
        }*/
        Option::None
    }

}