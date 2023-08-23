fn main() {
    // Array are stored on stack or binary & are fixed sized.
    // Hence, they are not stored on heap like vecs are.
    let a = [100; 10];
    // Values are deep copied by default no need of clone.
    takes_array(a);
    // a is still valid.
    println!("{:?}", a);

    // Vec are stored on heap & variable size.
    let mut b: Vec<i32> = vec![10, 12, 11, 15, 20];
    // Size is dynamic.
    b.push(100);
    // Values are moved by default need to use clone for deep copy.
    takes_vec(b);
    // b is invalid now, because its moved.
    // println!("{:?}", b);
}

fn takes_array(_a: [i32; 10]) {}

fn takes_vec(_b: Vec<i32>) {}
