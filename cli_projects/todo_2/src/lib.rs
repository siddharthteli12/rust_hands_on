use std::{fs::File, io::Read, process::exit};

struct TodoList {
    todos: Vec<Todo>,
}

struct Todo {
    description: String,
    is_completed: bool,
}

impl Todo {
    fn new(description: String, is_completed: bool) -> Self {
        Self {
            description,
            is_completed,
        }
    }
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
        let mut todo_list: Vec<Todo> = vec![];
        for todo in todos.lines() {
            if todo.chars().nth(0).unwrap() == '*' {
                todo_list.push(Todo::new(String::from(todo), true));
            } else {
                todo_list.push(Todo::new(String::from(todo), false));
            }
        }
        Self { todos: todo_list }
    }
}
