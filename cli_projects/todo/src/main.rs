use std::env;
use todo::*;
fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].to_ascii_lowercase().as_str() {
        "add" => {
            add_task();
        }
        "delete" => {
            delete_task();
        }
        "complete" => {
            complete_task();
        }
        "edit" => {
            edit_task();
        }
        "list" => {
            list_task();
        }
        _ => {}
    }
}
