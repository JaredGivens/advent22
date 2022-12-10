use std::{fs, cmp::Ordering};


fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let words = contents.split("\n").collect::<Vec<&str>>();
    let mut stashes: Vec<i64> = vec![0];
    let mut j = 0;
    for i in 0..words.len() {
        if let Ok(n) = words[i].parse::<i64>() {
            stashes[j] += n;
        } else {
            stashes.push(0);
            j += 1;
        }
    }
    let mut ans = 0;
    for i in 0..3 {
        let (index, max) = stashes.iter().enumerate().max_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap();
        ans += max;
        stashes.remove(index);
    }
    println!("{ans}");
}
