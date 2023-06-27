use std::{fs::File, io::Read, process::exit};

struct TodoList {
    todos: Vec<Todo>,
}

struct Todo {
    description: String,
    is_completed: bool,
}

impl TodoList {
    fn new(path: &str) -> Self {
        match File::open(path) {
            Ok(mut file) => {
                let mut todos: String = String::new();
                file.read_to_string(&mut todos).expect("Issue reading file");
                Self::build_todos(todos)
            }
            Err(e) => match e.kind() == std::io::ErrorKind::NotFound {
                true => {
                    File::create(path).expect("Issue creating file");
                    Self { todos: vec![] }
                }
                false => {
                    println!("Error opening file due to {:?}", e);
                    exit(1)
                }
            },
        }
    }

    fn build_todos(todos: String) -> Self {
        Self { todos: vec![] }
    }
}
