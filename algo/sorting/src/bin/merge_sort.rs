fn main() {
    let mut list = vec![100, 12, 12, 500, 213, 1, 2];
    println!("Sorted array {:?}", merge_sort(&mut list));
}

fn merge_sort(target: &mut Vec<i32>) -> Vec<i32> {
    if target.len() == 1 {
        return vec![target.pop().unwrap()];
    }
    let mid = target.len() / 2;
    let mut left_sorted = merge_sort(&mut target.drain(..mid).collect::<Vec<i32>>());
    let mut right_sorted = merge_sort(&mut target.drain(..).collect::<Vec<i32>>());
    let mut sorted_list = vec![];
    while !left_sorted.is_empty() && !right_sorted.is_empty() {
        if left_sorted[0] >= right_sorted[0] {
            sorted_list.push(right_sorted.remove(0))
        } else {
            sorted_list.push(left_sorted.remove(0))
        }
    }

    sorted_list.append(if !left_sorted.is_empty() {
        &mut left_sorted
    } else {
        &mut right_sorted
    });

    sorted_list
}
