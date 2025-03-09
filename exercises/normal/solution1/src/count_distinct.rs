use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    // todo!()
    let parts:Vec<_> = input_str.split(',').collect();
    let unique :HashSet<_> = parts.into_iter().collect();
    unique.len()
}
