use std::fs;

fn calco (c: &mut std::str::Chars) -> i32 {
    match c.nth(0).unwrap() {
        'A' => 3,
        'B' => 0,
        'C' => 6,
        _ => 0,
    }
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
                score += 1 + calco(&mut line);
            }
            'Y' => {
                score += 2 + calco(&mut line);
            }
            'Z' => {
                score += 3 + calco(&mut line);
            }
            _ => {}
        }
    }
    
    println!("{score}");

}
