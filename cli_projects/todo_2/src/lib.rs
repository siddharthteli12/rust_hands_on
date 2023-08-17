use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    process::exit,
};

use colored::Colorize;

#[derive(Clone)]
pub struct TodoList {
    todo_path: String,
    todos: Vec<Todo>,
}

#[derive(Clone)]

pub struct Todo {
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
    /// Creates todo list instance from file.
    /// Parameters:
    /// - `todo_path`: Todo file path to open/create.
    pub fn new(todo_path: String) -> Self {
        match File::open(&todo_path) {
            Ok(mut file) => {
                let mut todos: String = String::new();
                file.read_to_string(&mut todos).expect("Issue reading file");
                Self {
                    todo_path,
                    todos: Self::build_todos(todos),
                }
            }
            Err(e) => match e.kind() == std::io::ErrorKind::NotFound {
                true => {
                    File::create(&todo_path).expect("Issue creating file");
                    Self {
                        todo_path,
                        todos: vec![],
                    }
                }
                false => {
                    println!("Error opening file due to {:?}", e);
                    exit(1)
                }
            },
        }
    }

    /// Utility method to return list of todo.
    /// Parameters:
    /// - `todos`: Data read from todo file.
    fn build_todos(todos: String) -> Vec<Todo> {
        let mut todo_list: Vec<Todo> = vec![];
        for todo in todos.lines() {
            if let Some(todo_str) = todo.strip_prefix('*') {
                todo_list.push(Todo::new(todo_str.to_string(), true));
            } else {
                todo_list.push(Todo::new(String::from(todo), false));
            }
        }
        todo_list
    }

    /// Add new todo to instance.
    /// Parameters:
    /// - `todo`: New todo description.
    pub fn add_todo(&mut self, todo: String) {
        self.todos.push(Todo::new(todo, false))
    }

    /// Delete todo by index.
    /// Parameters:
    /// - `index`: Index of todo to delete.
    pub fn delete_todo(&mut self, index: usize) {
        self.todos.remove(index);
    }

    /// Edit todo with new_todo by index in instance.
    /// Parameters:
    /// - `index`: Index of todo to edit.
    /// - `new_todo`: New todo to add.
    pub fn edit_todo(&mut self, index: usize, new_todo: String) {
        self.todos[index] = Todo::new(new_todo, false);
    }

    /// List complete & uncompleted todo from instance.
    pub fn list_todo(&self) {
        for (index, todo) in self.todos.iter().enumerate() {
            if todo.is_completed {
                println!(
                    "{:}.) {:}",
                    index,
                    todo.description.strikethrough().bold().green()
                );
            } else {
                println!("{:}.) {:}", index, todo.description.bold().yellow());
            }
        }
    }

    /// Mark todo as complete by index.
    /// Parameters:
    /// - `index`: Index of todo to mark as complete.
    pub fn mark_todo_as_complete(&mut self, index: usize) {
        let mut todo = &mut self.todos[index];
        if todo.is_completed {
            todo.is_completed = true;
        } else {
            println!("Todo already marked as completed");
        }
    }

    /// Write todo instance to file.
    pub fn write_todos_to_file(&mut self) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .unwrap();
        let mut todos = String::new();
        for todo in self.todos.iter() {
            if todo.is_completed {
                todos.push_str(&format!("*{}\n", todo.description));
            } else {
                todos.push_str(&format!("{}\n", todo.description));
            }
        }
        file.write_all(todos.as_bytes())
            .expect("Issue writing to file");
    }
}
