#![allow(dead_code)]
struct Student {
    class: String,
    name: String,
    age: i8,
}

#[derive(Debug)]
struct Human {
    name: String,
    age: i8,
}

impl From<Student> for Human {
    fn from(value: Student) -> Self {
        Self {
            name: value.name,
            age: value.age,
        }
    }
}

fn main() {
    let student = Student {
        class: String::from("Engineering"),
        name: String::from("Siddharth"),
        age: 23,
    };

    //  student.into() invokes from method internally.
    let human = Human::from(student);
    println!("Human struct value from student {:?}", human);
}
