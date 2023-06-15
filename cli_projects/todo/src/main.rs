use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].to_ascii_lowercase().as_str() {
        "add" => {}
        "delete" => {}
        "complete" => {}
        "edit" => {}
        "list" => {}
        _ => {}
    }
}
