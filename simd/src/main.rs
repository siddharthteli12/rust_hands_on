#![feature(portable_simd)]

use packed_simd::*;

const LANES: usize = 16;
const LENGTH: usize = LANES * 60000;

fn main() {
    let data1 = [43; LENGTH];
    let data2 = [53; LENGTH];
    let now = std::time::Instant::now();
    normal_addition(&data1, &data2);
    let time1 = now.elapsed().as_millis();
    println!("Normal addition time - {}", time1);
    simd_addition(&data1, &data2);
    println!("Simd addition time - {}", now.elapsed().as_millis() - time1);
}

fn normal_addition(data1: &[i32], data2: &[i32]) {
    let mut result = vec![0; data1.len()];
    for i in 0..data1.len() {
        result[i] = data1[i] + data2[i];
    }
}

fn simd_addition(data1: &[i32], data2: &[i32]) {
    assert_eq!(data1.len(), data2.len(), "Array lengths must be equal");
    let mut result = vec![0; data1.len()];
    for i in (0..data1.len()).step_by(LANES) {
        let a_chunk = i32x16::from_slice_unaligned(&data1[i..]);
        let b_chunk = i32x16::from_slice_unaligned(&data2[i..]);
        let sum = a_chunk + b_chunk;
        sum.write_to_slice_unaligned(&mut result[i..]);
    }
}
