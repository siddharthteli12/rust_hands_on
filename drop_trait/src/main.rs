#![allow(unused_variables, dead_code, unused_assignments)]

struct CustomType(Vec<u8>);

impl Drop for CustomType {
    fn drop(&mut self) {
        println!("Drop is called");
    }
}

fn main() {
    let temp;
    {
        let custom_value1 = CustomType(vec![1, 2, 3]);
        let custom_value2 = CustomType(vec![1, 2, 3]);
        // Value moved, Hence, drop would not be called.
        temp = custom_value1;
    }
    println!("First Scope end");
}
