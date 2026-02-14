//private functions

use std::fs;

use rand::{rng, seq::{IndexedRandom}};
use serde_json::Value;

fn get_json_value(value_name: String) -> Vec<String> {
    let read_json = match fs::read_to_string("config/language.json") {
        Ok(data) => data,
        Err(why) => {
            eprintln!("Could not read language.json: {}", why);
            return vec![value_name];
        }
    };

    let json: Value = match serde_json::from_str(&read_json) {
        Ok(value) => value,
        Err(why) => {
            eprintln!("Could not parse json: {}", why);
            return vec![value_name];
        }
    };

    // Get array safely
    json.get(&value_name)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_else(|| vec![value_name])
}

//public functions
pub fn get_message_string(message_string_id: String) -> String {
    let language_strings = get_json_value(message_string_id);
    let selected_string: String = language_strings
        .choose(&mut rng())
        .cloned()
        .unwrap_or_else(|| "missing language string".to_string());

    return selected_string;
}
