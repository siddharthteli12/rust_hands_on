use std::{
    fs::OpenOptions,
    io::{Read, Seek, Write},
};

use colored::Colorize;

pub fn add_task(task: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    writeln!(file, "{:}", task).expect("Issue writing to file");
}

pub fn delete_task(index: usize) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("todo.txt")
        .unwrap();

    let mut tasks = String::new();
    file.read_to_string(&mut tasks)
        .expect("Issue reading to string");

    let tasks: String = tasks
        .lines()
        .enumerate()
        .into_iter()
        .filter(|&(i, _)| i != index)
        .map(|(_, e)| format!("{:}\n", e))
        .collect();

    file.set_len(0).expect("Issue in settin len to zero");
    file.rewind().expect("Issue rewinding");
    file.write(tasks.as_bytes()).expect("Issue writing to file");
}

pub fn complete_task(index: usize) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("todo.txt")
        .unwrap();

    let mut tasks = String::new();
    file.read_to_string(&mut tasks)
        .expect("Issue reading from file");
    let mut tasks: Vec<&str> = tasks.lines().collect();

    if tasks[index].starts_with("*") {
        return;
    } else {
        let complete_task = format!("*{:}", tasks[index]);
        tasks[index] = &complete_task;
        file.set_len(0).expect("Issue setting file len to 0");
        file.rewind().expect("Issue rewinding");
        file.write((tasks.join("\n") + "\n").as_bytes())
            .expect("Issue writing to file");
    }
}

pub fn edit_task(index: usize, new_task: String) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("todo.txt")
        .unwrap();
    let mut tasks = String::new();
    file.read_to_string(&mut tasks)
        .expect("Issue reading to string");

    let mut tasks: Vec<&str> = tasks.lines().collect();

    tasks[index] = &new_task;
    file.set_len(0).expect("Issue setting file len to 0");
    file.rewind().expect("Issue rewinding");
    file.write((tasks.join("\n") + "\n").as_bytes())
        .expect("Issue writing to file");
}

pub fn list_task() {
    let mut file = OpenOptions::new().read(true).open("todo.txt").unwrap();

    let mut tasks = String::new();
    file.read_to_string(&mut tasks)
        .expect("Issue reading to string");

    for (index, task) in tasks.lines().enumerate() {
        if task.chars().nth(0).unwrap() == '*' {
            println!(
                "{:} {:}",
                index.to_string().bold(),
                task[1..].strikethrough()
            );
        } else {
            println!("{:} {:}", index.to_string().bold(), task.green());
        }
    }
}
