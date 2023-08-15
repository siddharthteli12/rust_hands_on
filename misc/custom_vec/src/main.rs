use std::ptr::NonNull;

struct Vec1<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> Vec1<T> {
    fn new() -> Self {
        assert_ne!(std::mem::size_of::<T>(), 0);
        Self {
            ptr: NonNull::dangling(),
            cap: 0_usize,
            len: 0_usize,
        }
    }
}

fn main() {
    let vec_eg = Vec1::<i32>::new();
}
