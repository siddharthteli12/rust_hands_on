use std::collections::HashMap;

fn main() {
    let mut student_height: HashMap<&str, i32> = HashMap::new();
    student_height.insert("Foo", 170);
    student_height.insert("Boo", 180);

    for student_info in student_height {
        println!("{:?}", student_info);
    }
}
