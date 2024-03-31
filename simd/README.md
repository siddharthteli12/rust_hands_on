- Single instruction multiple data(SIMD)
    - It just means, we perform single instruction like add, mul, div, sub once on mulitple data rather than just two. 
    - It can make processing 4 times faster depending on num of data we are processing.
    - There's limited context where simd operations can we used.
    - Simd is cpu specific too, Mostly all cpu support simd instruction but have some varience.

-  Running code
    - `cargo +nightly run`
    - The simd addition is twice has fast with normal addition.
