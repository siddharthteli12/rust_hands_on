//! This crate is for learning lifetime concepts.
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Store remaining & delimiter slice.
#[derive(Debug)]
pub struct StrSpilt<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSpilt<'a> {
    #[allow(dead_code)]
    fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSpilt<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delimeter) = remainder.find(self.delimiter) {
                let result = &remainder[..next_delimeter];
                *remainder = &remainder[next_delimeter + self.delimiter.len()..];
                Some(result)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

#[test]
fn delimiter_spaced_string() {
    let haystack = "a b c d e";
    let letters: Vec<&str> = StrSpilt::new(haystack, " ").collect();
    // Eq converts letters to iterator type.
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail_empty_string() {
    let haystack = "a b c d e ";
    let letters: Vec<&str> = StrSpilt::new(haystack, " ").collect();
    // Eq converts letters to iterator type.
    assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
}
