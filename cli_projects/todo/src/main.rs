use std::env;
use todo::*;

const MESSAGE: &str = "Incorrect subcommand only accepect-\nadd\ndelete\ncomplete\nedit\nlist";

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].to_ascii_lowercase().as_str() {
        "add" => {
            add_task(args[2..].join(" "));
        }
        "delete" => {
            delete_task(args[2].parse::<usize>().unwrap());
        }
        "complete" => {
            complete_task(args[2].parse::<usize>().unwrap());
        }
        "edit" => {
            edit_task(args[2].parse::<usize>().unwrap(), args[3..].join(" "));
        }
        "list" => {
            list_task();
        }
        _ => {
            println!("{MESSAGE}")
        }
    }
}
