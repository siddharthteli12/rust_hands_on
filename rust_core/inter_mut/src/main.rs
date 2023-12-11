use std::cell::Cell;

trait Info {
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_age(&self) -> u32;
}

struct Student {
    name: String,
    age: u32,
    address: String,
    read_count: Cell<u32>,
}

impl Student {
    fn new(name: String, age: u32, address: String) -> Self {
        Self {
            name,
            age,
            address,
            read_count: Cell::new(0),
        }
    }
}

impl Info for Student {
    fn get_address(&self) -> &str {
        self.read_count.set(self.read_count.get() + 1);
        self.address.as_str()
    }
    fn get_name(&self) -> &str {
        self.read_count.set(self.read_count.get() + 1);
        self.name.as_str()
    }
    fn get_age(&self) -> u32 {
        self.read_count.set(self.read_count.get() + 1);
        self.age
    }
}

fn main() {
    let student = Student::new(String::from("Siddharth"), 24, String::from("Hyd"));
    student.get_address();
    student.get_name();
    println!("Value of counter - {:?}", student.read_count.into_inner());
}
