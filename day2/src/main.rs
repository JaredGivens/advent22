use std::fs;

fn part_b(line: &str) ->i64 {
    let mut score = 0;
    match line.chars().nth(2).unwrap() {
        'X' => {
            score += match line.chars().nth(0).unwrap() {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => 0,
            }
        }
        'Y' => {
            score += 3 + match line.chars().nth(0).unwrap() {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => 0,
            }
        }
        'Z' => {
            score += 6 + match line.chars().nth(0).unwrap() {
                'A' => 2,
                'B' => 3,
                'C' => 1,
                _ => 0,
            }
        }
        _ => {}
    }
    score

}
fn part_a(line: &str) -> i64 {
    let mut score = 0;

    match line.chars().nth(2).unwrap() {
        'X' => {
            score += 1 + match line.chars().nth(0).unwrap() {
                'A' => 3,
                'B' => 0,
                'C' => 6,
                _ => 0,
            }
        }
        'Y' => {
            score += 2 + match line.chars().nth(0).unwrap() {
                'A' => 6,
                'B' => 3,
                'C' => 0,
                _ => 0,
            }
        }
        'Z' => {
            score += 3 + match line.chars().nth(0).unwrap() {
                'A' => 0,
                'B' => 6,
                'C' => 3,
                _ => 0,
            }
        }
        _ => {}
    }
    score

}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut partA = 0;
    let mut partB = 0;
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            continue;
        }
        println!("{}", lines[i]);
        partA += part_a(lines[i]);
        partB += part_b(lines[i]);
    }
    
    println!("part a: {partA}");
    println!("part b: {partB}");

}
