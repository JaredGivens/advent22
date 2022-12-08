use std::fs;

fn calco (c: &mut std::str::Chars) -> i32 {
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut score = 0;
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            continue;
        }
        println!("{}", lines[i]);
        let mut line = lines[i].chars();

        match line.nth(2).unwrap() {
            'X' => {
                score += 1 + match c.nth(0).unwrap() {
                    'A' => 3,
                    'B' => 0,
                    'C' => 6,
                    _ => 0,
                }
            }
            'Y' => {
                score += 2 + match c.nth(0).unwrap() {
                    'A' => 6,
                    'B' => 3,
                    'C' => 0,
                    _ => 0,
                }
            }
            'Z' => {
                score += 3 + match c.nth(0).unwrap() {
                    'A' => 0,
                    'B' => 6,
                    'C' => 3,
                    _ => 0,
                }
            }
            _ => {}
        }
    }
    
    println!("{score}");

}
