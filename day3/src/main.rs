use std::fs;

fn to_prio(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    let mut group_sum = 0;
    let mut group = Vec::<String>::new();
    
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            continue;
        }

        group.push(String::from(lines[i]));
        if (i + 1) % 3 == 0 {

            let mut j: usize = 0;
            let mut k: usize = 0;
            let mut l: usize = 0;

            let mut i0 = '0';
            let mut i1 = '1';
            let mut i2 = '2';
            loop {
                i0 = group[0].chars().nth(j).unwrap();
                i1 = group[1].chars().nth(k).unwrap();
                i2 = group[2].chars().nth(l).unwrap();

                if i0 == i1 && i0 == i2 {
                    break;
                }

                j += 1;
                if j == group[0].len() {
                    k += 1;
                    if k == group[1].len() {
                        l += 1;
                        k = 0;
                    }
                    j = 0;
                }
            }
            let c = group[0].chars().nth(j).unwrap();
            group_sum += to_prio(c);
            group.clear();
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

        sum += to_prio(c);
        
    }
    println!("{group_sum} {sum}");
}
