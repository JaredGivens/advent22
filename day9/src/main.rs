use std::{collections::HashSet, fs};
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn clone(&self) -> Self {
        Self { x: self.x, y: self.y }
    }
    pub fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
    pub fn follow(&mut self, lead: &Vec2) {
        let d = (lead.x - self.x, lead.y - self.y);
        match d {
            (-1..=1, -1..=1) => {}
            (0, -2..=2) => {
                self.y += d.1.signum();
            }
            (-2..=2, 0) => {
                self.x += d.0.signum();
            }
            _ => {
                self.x += d.0.signum();
                self.y += d.1.signum();
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut ans = HashSet::<String>::new();
    let mut head = Vec2::new();
    let mut tails = Vec::<Vec2>::new();
    for _ in 0..9 {
        tails.push(Vec2::new());
    }

    ans.insert(tails[0].to_string());

    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }
        let move_end = match line.chars().next().unwrap() {
            'L' => |v: &mut Vec2| v.x -= 1,
            'R' => |v: &mut Vec2| v.x += 1,
            'D' => |v: &mut Vec2| v.y -= 1,
            'U' => |v: &mut Vec2| v.y += 1,
            _ => |v: &mut Vec2| {},
        };
        for _ in 0..line[2..].parse::<i32>().unwrap() {
            move_end(&mut head);
            tails[0].follow(& head);
            for i in 1..tails.len() {
                let mut t0 = tails[i].clone();
                t0.follow(& tails[i - 1]);
                tails[i] = t0;
            }
            ans.insert(tails[tails.len() - 1].to_string());
        }
    }
    println!("{}", ans.len());
}
