use std::env;
use todo_2::*;

const MESSAGE: &str =
    "Incorrect subcommand, only accepted subcommand is-\nadd\ndelete\ncomplete\nedit\nlist";
const TODO_PATH: &str = "todo.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(arg) => {
            let mut todo_list = TodoList::new(String::from(TODO_PATH));
            match arg.to_ascii_lowercase().as_str() {
                "add" => {
                    todo_list.add_todo(args[2..].join(" "));
                }
                "delete" => {
                    todo_list.delete_todo(args[2].parse::<usize>().unwrap());
                }
                "complete" => {
                    todo_list.mark_todo_as_complete(args[2].parse::<usize>().unwrap());
                }
                "edit" => {
                    todo_list.edit_todo(args[2].parse::<usize>().unwrap(), args[3..].join(" "));
                }
                "list" => {
                    todo_list.list_todo();
                }
                _ => {
                    println!("{MESSAGE}")
                }
            };
            todo_list.write_todos_to_file();
        }
        None => {
            println!("Missing cli arguments")
        }
    }
}
