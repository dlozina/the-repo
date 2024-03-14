use serde_json::Value;
use std::fs;
use log::{info, warn};

fn main() {
    // Initialize the logger
    env_logger::init();

    // Example of reading and parsing JSON files
    //"path/to/your/first.json"
    //"path/to/your/second.json"
    let json_str1 = fs::read_to_string("/Users/dlozina/workspace/NBA POC DATA/latest NBA data schema/test diff/macrometa_pbp_0022300336.json").expect("Failed to read first file");
    let json_str2 = fs::read_to_string("/Users/dlozina/workspace/NBA POC DATA/latest NBA data schema/test diff/macrometa_pbp_0022300336 copy.json").expect("Failed to read second file");

    let json1: Value = serde_json::from_str(&json_str1).expect("Failed to parse first JSON");
    let json2: Value = serde_json::from_str(&json_str2).expect("Failed to parse second JSON");

    let differences = compare_json(&json1, &json2, "");
    if differences.is_empty() {
        info!("No differences found");
    } else {
        for diff in differences {
            warn!("{}", diff); // Logging differences as warnings
        }
    }
}

fn compare_json(a: &Value, b: &Value, path: &str) -> Vec<String> {
    let mut differences = Vec::new();
    match (a, b) {
        (Value::Object(map1), Value::Object(map2)) => {
            for (key, val1) in map1 {
                let new_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
                let val2 = map2.get(key);
                differences.extend(compare_json(val1, &val2.unwrap_or(&Value::Null), &new_path));
            }
            // Check for keys in b that are not in a
            for key in map2.keys() {
                if !map1.contains_key(key) {
                    let new_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
                    differences.push(format!("Key '{}' found in the second JSON but not in the first. Path: {}", key, new_path));
                }
            }
        },
        (Value::Array(arr1), Value::Array(arr2)) => {
            for (index, item) in arr1.iter().enumerate() {
                if index < arr2.len() {
                    let new_path = format!("{}[{}]", path, index);
                    differences.extend(compare_json(item, &arr2[index], &new_path));
                } else {
                    let new_path = format!("{}[{}]", path, index);
                    differences.push(format!("Missing item at index {} in the second JSON. Path: {}", index, new_path));
                }
            }
            // Check if the second array has more items than the first
            if arr2.len() > arr1.len() {
                for extra_index in arr1.len()..arr2.len() {
                    let new_path = format!("{}[{}]", path, extra_index);
                    differences.push(format!("Additional item at index {} in the second JSON. Path: {}", extra_index, new_path));
                }
            }
        },
        // This will handle all types of values (Strings, Numbers, etc.)
        _ => {
            if a != b {
                differences.push(format!("Difference found at path '{}': '{}' (first) vs '{}' (second)", path, a, b));
            }
        }
    }
    differences
}
