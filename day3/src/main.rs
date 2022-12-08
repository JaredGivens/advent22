use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            continue;
        }
        let mid = lines[i].len() / 2;
        let c0 = &lines[i][..mid];
        let c1 = &lines[i][mid..];
        let mut j: usize = 0;
        let mut k: usize = 0;
        while c0.chars().nth(j).unwrap() != c1.chars().nth(k).unwrap() {
            k += 1;
            if k == c1.len() {
                j += 1;
                k = 0;
            }
        }
        let c = c0.chars().nth(j).unwrap();
        let p = match c {
            'a'..='z' => c as u32 - 'a' as u32 + 1,
            'A'..='Z' => c as u32 - 'A' as u32 + 27,
            _ => 0,
        };

        println!("{c} {p} {:?}, {:?}", c0, c1);
        sum += p;
        
    }
    println!("{sum}");
}
