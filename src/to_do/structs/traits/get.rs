use serde_json::Map;
use serde_json::value::Value;


pub trait Get {
    fn get_title(&self) -> &str;

    fn get(&self, state: Map<String, Value>) {
        let item = state.get(self.get_title());
        match item {
            Some(result) => {
                println!("{} -> {}", self.get_title(), result);
            },
            None => println!("Item was not found: {}", self.get_title())
        }
    }
}
