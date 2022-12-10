use std::{fs, collections::HashSet};
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut ans = HashSet::<String>::new();
    let mut head = Vec2::new();
    let mut tail = Vec2::new();
    ans.insert(tail.to_string());

    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }
        let move_end = match line.chars().next().unwrap() {
            'L' => |v: &mut Vec2| { v.x -=1 },
            'R' => |v: &mut Vec2| { v.x +=1 },
            'D' => |v: &mut Vec2| { v.y -=1 },
            'U' => |v: &mut Vec2| { v.y +=1 },
            _ => |v: &mut Vec2| {},
        };
        for _ in 0..line[2..].parse::<i32>().unwrap() {
            move_end(&mut head);
            let d = (head.x - tail.x, head.y - tail.y);
            match d {
                (-1..=1, -1..=1) => {},
                (0, -2..=2) => {
                    tail.y += d.1.signum();
                },
                (-2..=2, 0) => {
                    tail.x += d.0.signum();
                },
                _ => {
                    tail.x += d.0.signum();
                    tail.y += d.1.signum();
                }
            }
            ans.insert(tail.to_string());
        }
    }
    println!("{}", ans.len());
}

