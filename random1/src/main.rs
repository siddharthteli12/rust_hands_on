unsafe fn modify_pointer(pointer: *mut Vec<i32>) {
    *pointer = vec![5, 6, 7, 8];
}

unsafe fn modify_pointer_box(pointer: *mut Box<i32>) {
    *pointer = Box::new(10);
    println!("Box pointer - {:?}", pointer);
}

unsafe fn modify_pointer_human(pointer: *mut Human) {
    *pointer = Human {
        name: "Dumb".to_string(),
        age: 100,
    };
    println!("Human pointer - {:?}", pointer);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Human {
    name: String,
    age: i32,
}

fn main() {
    let mut a: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut b = Box::new(10);
    let mut human = Human {
        name: String::from("Sid"),
        age: 23,
    };
    unsafe {
        modify_pointer(&mut a);
        modify_pointer_box(&mut b);
        modify_pointer_human(&mut human);
    }
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", human);
}
