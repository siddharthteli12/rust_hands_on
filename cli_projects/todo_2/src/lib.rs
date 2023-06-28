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

    fn build_todos(todos: String) -> Vec<Todo> {
        let mut todo_list: Vec<Todo> = vec![];
        for todo in todos.lines() {
            if todo.chars().nth(0).unwrap() == '*' {
                todo_list.push(Todo::new(String::from(todo)[1..].to_string(), true));
            } else {
                todo_list.push(Todo::new(String::from(todo), false));
            }
        }
        todo_list
    }

    pub fn add_todo(&mut self, todo: String) {
        self.todos.push(Todo::new(todo, false))
    }

    pub fn delete_todo(&mut self, index: usize) {
        self.todos.remove(index);
    }

    pub fn edit_todo(&mut self, index: usize, new_todo: String) {
        self.todos[index] = Todo::new(new_todo, false);
    }

    pub fn list_todo(&self) {
        let mut counter = 0;
        for todo in &self.todos {
            if todo.is_completed {
                println!(
                    "{:}.) {:}",
                    counter,
                    todo.description.strikethrough().bold().green()
                );
            } else {
                println!("{:}.) {:}", counter, todo.description.bold().yellow());
            }
            counter = counter + 1;
        }
    }

    pub fn mark_todo_as_complete(&mut self, index: usize) {
        let is_completed = &mut self.todos[index].is_completed;
        if !*is_completed {
            *is_completed = true;
        } else {
            println!("Todo already marked as completed");
        }
    }

    pub fn write_todos_to_file(&mut self) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .unwrap();
        let mut todos = String::new();
        for mut todo in self.todos.clone() {
            if todo.is_completed {
                todo.description.insert(0, '*');
                todos.push_str((todo.description + "\n").as_str());
            } else {
                todos.push_str((todo.description + "\n").as_str());
            }
        }
        file.write(todos.as_bytes()).expect("Issue writing to file");
    }
}
