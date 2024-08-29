mod to_do;
mod state;
mod process;

use std::env;

const STATE_FILE_NAME: &str = "state.json";

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: &String = &args[1];
    let title: &String = &args[2];

    let state = state::load_state_from_file(STATE_FILE_NAME);
    let status: String;

    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace("\"", "");
        }
        _ => status = String::from("pending")
    }
    println!("status = {}", status);
    let result = to_do::to_do_factory(status.as_str(), title);
    println!("result = {:?}", result);
    match result{
        Ok(item) => process::process_input(item, command.to_string(), state, STATE_FILE_NAME),
        Err(err) => eprintln!("Error item processing: {:?}", err)
    }
}
