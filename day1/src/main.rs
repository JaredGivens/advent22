use std::fs;
fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut reci = 0;
    let mut rec = 0;
    let mut cur = 0;
    let words = contents.split("\n").collect::<Vec<&str>>();
    for i in 0..words.len() {
        if let Ok(n) = words[i].parse::<i64>() {
            cur += n;
        } else {
            println!("{cur}");
            if cur > rec {
                rec = cur;
                reci = i;
            }
            cur = 0;
        }
    }

    println!("{rec}");

}
