unsafe fn modify_pointer(pointer: *mut i32) -> i32 {
    *pointer = 10 as i32;
    *pointer
}

fn main() {
    let mut a = 12;
    unsafe {
        println!("{:}", modify_pointer(&mut a));
    }
}
