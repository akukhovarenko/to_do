mod to_do;
mod state;

use to_do::{to_do_factory, ItemTypes};

fn main() {
    let item = to_do_factory("done", "any_title_done");
    match item {
        Ok(ItemTypes::Done(item_type)) => item_type.super_struct.print(),
        Ok(ItemTypes::Pending(item_type)) => item_type.super_struct.print(),
        Err(message) => eprintln!("ERROR: {}", message)

    }
}
