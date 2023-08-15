//! This crate is for learning lifetime concepts.
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Store remaining & delimiter slice.
#[derive(Debug)]
pub struct StrSpilt<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSpilt<'a, D> {
    #[allow(dead_code)]
    fn new(haystack: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

trait Delimeter {
    fn find_next(&self, target: &str) -> Option<(usize, usize)>;
}

impl<'a, D> Iterator for StrSpilt<'a, D>
where
    D: Delimeter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some((start, end)) = self.delimiter.find_next(remainder) {
                let result = &remainder[..start];
                *remainder = &remainder[end..];
                Some(result)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimeter for &str {
    fn find_next(&self, target: &str) -> Option<(usize, usize)> {
        target
            .find(self)
            .map(|start_index| (start_index, start_index + self.len()))
    }
}

impl Delimeter for char {
    fn find_next(&self, target: &str) -> Option<(usize, usize)> {
        target
            .find(*self)
            .map(|start_index| (start_index, start_index + 1))
    }
}

#[allow(dead_code)]
fn until_char<T: Delimeter>(target: &str, delimeter: T) -> Option<&str> {
    StrSpilt::new(target, delimeter).next()
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

#[test]
fn until_first_char() {
    let target = "Hey how are you";
    let delimeter = 'y';
    assert_eq!(until_char(target, delimeter), Some("He"));
}
