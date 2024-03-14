use serde_json::{json, Value};
use std::fs;
use std::time::Instant;
use log::{info, warn};

fn main() {
    // Initialize the logger
    env_logger::init();

    // Example of reading and parsing JSON files
    // Before commit
    //"path/to/your/first.json"
    //"path/to/your/second.json"
    let json_str1 = fs::read_to_string("/Users/dlozina/workspace/NBA POC DATA/latest NBA data schema/test diff/macrometa_pbp_0022300336.json").expect("Failed to read first file");
    let json_str2 = fs::read_to_string("/Users/dlozina/workspace/NBA POC DATA/latest NBA data schema/test diff/macrometa_pbp_0022300336 copy.json").expect("Failed to read second file");

    let json1: Value = serde_json::from_str(&json_str1).expect("Failed to parse first JSON");
    let json2: Value = serde_json::from_str(&json_str2).expect("Failed to parse second JSON");

    // Start timing
    let start = Instant::now();
    let differences = compare_json(&json1, &json2);
    // End timing
    let duration = start.elapsed();

    // Convert differences to a JSON string for display or storage
    let diffs_json_str = serde_json::to_string_pretty(&differences).expect("Failed to serialize differences");
    println!("{}", diffs_json_str);

    // Print out the execution time
    println!("Execution time: {} ms", duration.as_millis());
}

fn compare_json(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Object(map1), Value::Object(map2)) => {
            let mut diffs = serde_json::Map::new();
            for (key, val1) in map1 {
                let val2 = map2.get(key).unwrap_or(&Value::Null);
                let child_diffs = compare_json(val1, val2);
                if child_diffs != Value::Null {
                    diffs.insert(key.clone(), child_diffs);
                }
            }
            // Here we don't check the keys from the first JSON absent in the second,
            // since we only want to display changes that exist in the second JSON
            if diffs.is_empty() { Value::Null } else { Value::Object(diffs) }
        },
        (Value::Array(arr1), Value::Array(arr2)) => {
            let mut diffs = vec![];
            let max_len = std::cmp::max(arr1.len(), arr2.len());
            for i in 0..max_len {
                let val1 = arr1.get(i).unwrap_or(&Value::Null);
                let val2 = arr2.get(i).unwrap_or(&Value::Null);
                let child_diffs = compare_json(val1, val2);
                if child_diffs != Value::Null {
                    // Here we only push changes from the second array
                    diffs.push(child_diffs);
                }
            }
            if diffs.is_empty() { Value::Null } else { Value::Array(diffs) }
        },
        _ => {
            if a != b {
                b.clone() // Instead of showing "changed_from" and "changed_to", only show the new value
            } else {
                Value::Null // No difference
            }
        }
    }
}


