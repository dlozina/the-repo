use serde_json::Value;
use std::fs;
use log::{info, warn};

fn main() {
    // Initialize the logger
    env_logger::init();

    // Example of reading and parsing JSON files
    let json_str1 = fs::read_to_string("path/to/your/first.json").expect("Failed to read first file");
    let json_str2 = fs::read_to_string("path/to/your/second.json").expect("Failed to read second file");

    let json1: Value = serde_json::from_str(&json_str1).expect("Failed to parse first JSON");
    let json2: Value = serde_json::from_str(&json_str2).expect("Failed to parse second JSON");

    let differences = compare_json(&json1, &json2);
    if differences.is_empty() {
        info!("No differences found");
    } else {
        for diff in differences {
            warn!("{}", diff); // Logging differences as warnings
        }
    }
}

fn compare_json(a: &Value, b: &Value) -> Vec<String> {
    let mut differences = Vec::new();
    if let (Some(map1), Some(map2)) = (a.as_object(), b.as_object()) {
        for (key, val1) in map1 {
            match map2.get(key) {
                Some(val2) if val1 != val2 => {
                    differences.push(format!("Difference found at key '{}': '{}' (first) vs '{}' (second)", key, val1, val2));
                },
                None => differences.push(format!("Key '{}' found in the first JSON but not in the second", key)),
                _ => {} // No difference or same values
            }
        }
        // Optionally check for keys in the second JSON that are not in the first
        for key in map2.keys() {
            if !map1.contains_key(key) {
                differences.push(format!("Key '{}' found in the second JSON but not in the first", key));
            }
        }
    } else {
        warn!("One of the JSONs is not an object");
    }
    differences
}
