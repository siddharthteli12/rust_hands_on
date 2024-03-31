#![feature(portable_simd)]
use std::simd::*;
use std::time::Instant;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut data1 = [43; 100000]; 
    let mut data2 = [53; 100000];
    let now = Instant::now();
    normal_addition(&mut data1, &mut data2);
    let time1 = now.elapsed().as_secs();
    println!("Normal addition time - {time1}");
    simd_addition(&mut data1, &mut data2);
    println!("Simd addition time - {}", now.elapsed().as_secs() - time1);
}


fn normal_addition(data1: &mut [i32], data2: &mut [i32]) {
    sleep(Duration::new(2, 0));

}

fn simd_addition(data1: &mut [i32], data2: &mut [i32]) {
    sleep(Duration::new(1, 0));
}
