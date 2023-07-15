//! This crate is for learning lifetime concepts.
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Store remaining & delimiter slice.
#[derive(Debug)]
pub struct StrSpilt<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSpilt<'a> {
    fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSpilt<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let result_str = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(result_str)
        } else if self.remainder.is_empty() {
            None
        } else {
            let result_str = self.remainder;
            self.remainder = "";
            Some(result_str)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSpilt::new(haystack, " ");
    // Eq converts letters to iterator type.
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"]));
}
