use packed_simd::*;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;

const LANES: usize = 16;
const BUFFER_SIZE: usize = 512 * 16;

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    process_json("input.json", "output.json")?;
    println!("Time taken - {:?}", now.elapsed().as_secs());
    Ok(())
}

fn process_json(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;
    let mut buffer = [0_u8; BUFFER_SIZE];
    loop {
        let read_bytes = input_file.read(&mut buffer).unwrap();
        if !read_bytes % LANES == 0 {
            break;
        }
        let result = process_json_chunks(&buffer[..read_bytes]);
        output_file.write(&result).unwrap();
    }
    Ok(())
}

fn process_json_chunks(list: &[u8]) -> Vec<u8> {
    let mut result = vec![0; list.len()];

    for i in (0..list.len()).step_by(LANES) {
        let a = u8x16::from_slice_unaligned(&list[i..]);
        let b = u8x16::splat(b';');
        let mask = a.eq(b);
        let replaced = mask.select(u8x16::splat(b':'), a);
        replaced.write_to_slice_unaligned(&mut result[i..]);
    }
    result
}
