use packed_simd::*;
const LANES: usize = 4;

fn main() {
    let list = [
        '{', ';', 'b', ';', ';', ',', ':', '/', '{', ';', 'b', ';', ';', ',', ':', '/',
    ];
    let to_replace = ';';
    let replace_with = ':';
    let input = list.iter().map(|value| *value as u8).collect();
    let result = process_list(input, to_replace as u8, replace_with as u8);
    let result_str: Vec<char> = result.iter().map(|value| *value as char).collect();
    println!("{:?}", result_str);
}

fn process_list(list: Vec<u8>, to_replace: u8, replace_with: u8) -> Vec<u8> {
    assert_eq!(list.len() % LANES, 0);
    let mut result = vec![0; list.len()];
    for i in (0..list.len()).step_by(LANES) {
        let a = u8x4::from_slice_aligned(&list[i..]);
        let b = u8x4::splat(to_replace);
        let mask = a.eq(b);
        let replaced = mask.select(u8x4::splat(replace_with), a);
        replaced.write_to_slice_unaligned(&mut result[i..]);
    }
    result
}
