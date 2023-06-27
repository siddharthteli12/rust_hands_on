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

    fn add_todo(&mut self, todo: String) {
        self.todos.push(Todo::new(todo, false))
    }

    fn delete_todo(&mut self, index: usize) {
        self.todos.remove(index);
    }

    fn edit_todo(&mut self, new_todo: String, index: usize) {
        self.todos[index] = Todo::new(new_todo, false);
    }

    fn mark_todo_as_complete(&mut self, index: usize) {
        let is_completed = &mut self.todos[index].is_completed;
        if !*is_completed {
            *is_completed = true;
        } else {
            println!("Todo already marked as completed");
        }
    }
}
