#![allow(dead_code)]

pub trait CustomIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Clone, Default, Debug, PartialEq)]
struct Human {
    age: i32,
    name: String,
    height: i32,
}

impl Human {
    fn new(age: i32, name: String, height: i32) -> Self {
        Self { age, name, height }
    }
}

#[derive(Clone)]
struct IterWrapper {
    human: Vec<Human>,
    index: usize,
}

impl IterWrapper {
    fn new(human: Vec<Human>) -> Self {
        Self {
            human: human,
            index: 0_usize,
        }
    }

    fn append(&mut self, age: i32, name: String, height: i32) {
        self.human.push(Human::new(age, name, height))
    }
}

impl CustomIterator for IterWrapper {
    type Item = Human;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.index;
        match self.human.get(current) {
            Some(val) => {
                self.index = current + 1;
                Some(val.clone())
            }
            None => None,
        }
    }
}

fn main() {
    let human_list = vec![
        Human::new(12, "Sid".to_string(), 170),
        Human::new(23, "AKhil".to_string(), 170),
    ];
    let mut sample_iter = IterWrapper::new(human_list);
    println!("Index 0 {:?}", sample_iter.next().unwrap_or_default());
    println!("Index 1 {:?}", sample_iter.next().unwrap_or_default());
    println!("Index 2 {:?}", sample_iter.next().unwrap_or_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_iter() {
        let mut test_sample_iterator = IterWrapper::new(vec![]);
        assert!(test_sample_iterator.next().is_none());
    }

    #[test]
    fn test_with_simple_iter() {
        let test_human_list = vec![Human::new(100, "test".to_string(), 1000)];
        let mut test_sample_iterator = IterWrapper::new(test_human_list.clone());
        assert_eq!(test_sample_iterator.next().unwrap(), test_human_list[0]);
        assert!(test_sample_iterator.next().is_none());
    }

    #[test]
    fn test_with_simple_iter_2() {
        let test_human_list = vec![
            Human::new(100, "test".to_string(), 1000),
            Human::new(200, "test2".to_string(), 2000),
        ];
        let mut test_sample_iterator = IterWrapper::new(test_human_list.clone());
        assert_eq!(test_sample_iterator.next().unwrap(), test_human_list[0]);
        assert_eq!(test_sample_iterator.next().unwrap(), test_human_list[1]);
        assert!(test_sample_iterator.next().is_none());
    }
}
