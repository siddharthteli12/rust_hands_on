use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

use colored::Colorize;

pub fn add_task(task: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{:}", task) {
        eprintln!("Issue writing to file due to {:}", e);
    }
}

pub fn delete_task(index: usize) {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    let mut tasks = String::new();
    let _ = file.read_to_string(&mut tasks);
    let result_task: Vec<&str> = tasks
        .lines()
        .enumerate()
        .into_iter()
        .filter(|&(i, _)| i != index)
        .map(|(_, e)| e)
        .collect();

    let _ = file.set_len(0);
    for task in result_task {
        let _ = file.write(format!("{:}\n", task).as_bytes());
    }
}

pub fn complete_task(index: usize) {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    let mut result_tasks = String::new();
    let _ = file.read_to_string(&mut result_tasks);
    let mut tasks: Vec<&str> = result_tasks.lines().collect();

    if tasks[index].starts_with("*") {
        return;
    } else {
        let completed_task = format!("*{:}", tasks[index]);
        tasks[index] = &completed_task;
        let _ = file.set_len(0);
        for task in tasks {
            let _ = file.write(format!("{:}\n", task).as_bytes());
        }
    }
}

pub fn edit_task() {}

pub fn list_task() {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open("todo.txt")
        .unwrap();

    let mut result_tasks = String::new();
    let _ = file.read_to_string(&mut result_tasks);

    for (index, task) in result_tasks.lines().enumerate() {
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
