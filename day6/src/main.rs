use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let min = 14;
    for i in (min - 1)..contents.len() {
        let mut can = HashSet::new();
        for j in 0..min {
            can.insert(contents.chars().nth(i - j).unwrap());
        }
        if can.len() == min {
            println!("{}", i);
            break;
        }
    }
}
