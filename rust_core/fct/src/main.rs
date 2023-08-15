fn main() {
    // x is not a pointer but its type is function item.
    // which is a zero sized value.
    let x = bar::<i32>;
    // This confirms that this is just a identifier not a pointer.
    // Because pointer cannot be zero sized.
    println!("{:?}", std::mem::size_of_val(&x));

    // baz takes function pointer as parameter.
    // x is a function item but it can be coerced to a function pointer but vice versa not.
    baz(x);
    baz(bar::<u128>);

    quox(bar::<u128>)
}

// x is of function pointer type.
// Function pointer impl Fn, which impl FnMut & it impls FnOnce.
// Hence, function pointer impl all three trait.
fn baz<T>(x: fn(T) -> T) {
    // Here, x is a pointer because its size is not zero but usize.
    println!("{:?}", std::mem::size_of_val(&x));
}

fn bar<T>(x: T) -> T {
    x
}

fn quox<T, F>(f: F)
where
    F: Fn(T) -> T,
{
}
