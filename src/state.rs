use std::fs;
use std::fs::File;
use std::io::Read;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();

    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state;
}

pub fn save_to_file(file_name: &str, state: Map<String, Value>) -> () {
    let data = json!(state);
    fs::write(file_name.to_string(), data.to_string()).expect("Unable to write data to file");
}


#[cfg(test)]
mod base_test {
    use serde_json::{Map, json};
    use super::{read_file, save_to_file};
    use tempfile::tempdir;

    #[test]
    fn read_write_to_file(){
        let dir = tempdir().unwrap();
        let binding = dir.path().join("tempfile.json");
        let file_name = binding.to_str().unwrap();
        let mut state = Map::new();
        state.insert(String::from("key"), json!("value"));
        save_to_file(file_name, state);
        let result = read_file(file_name);
        println!("{:?}", result);
        dir.close().unwrap();
    }
}