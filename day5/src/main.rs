use std::fs;
use std::cmp;
use std::result::Result;

struct Crates {
    mat: Vec<Vec<char>>
}

impl Crates {
    pub fn new() -> Self {
        Self {
            mat: Vec::new()
        }
    }
    pub fn push(&mut self, l: &str) -> bool {
        if let Some(mut i) = l.find(|c| c == '[') {
            loop {
                let col = i / 4;
                self.mat.resize(cmp::max(self.mat.len(),col + 1), 
                                   Vec::new());
                self.mat[col].insert(0, l.chars().nth(i + 1).unwrap());
                
                println!("{:?}", i);
                i += 1 + match l[i + 1..].find(|c| c == '[') {
                    Some(n) => n,
                    None => break,
                };
            }
            
            true
        } else {
            false
        }
    }
    pub fn parse_cmd9001(&mut self, s: &str) {
        let mut crane = Vec::new();
        let args = s.split(' ')
            .filter_map(|s| match s.parse::<usize>() {
                Ok(n) => Some(n),
                Result::Err(_) => None,
            }).collect::<Vec<usize>>();
        for _ in 0..args[0] {
            if let Some(c) = self.mat[args[1] - 1].pop() {
                crane.push(c);
            }
        }
        for _ in 0..args[0] {
            self.mat[args[2] - 1].push(crane.pop().unwrap());
        }
    }
    pub fn parse_cmd(&mut self, s: &str) {
        let args = s.split(' ')
            .filter_map(|s| match s.parse::<usize>() {
                Ok(n) => Some(n),
                Result::Err(_) => None,
            }).collect::<Vec<usize>>();
        for _ in 0..args[0] {
            if let Some(c) = self.mat[args[1] - 1].pop() {
                self.mat[args[2] - 1].push(c);
            }
        }
    }
    pub fn print(&mut self) {
        for i in 0..self.mat.len() {
            if let Some(n) = self.mat[i].last() {
                print!("{n} ");
            }
        }
        println!();
    }
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split('\n').collect::<Vec<&str>>();
    let mut crates = Crates::new();
    let mut crates9001 = Crates::new();
    let mut filled = false;
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        
        if !filled {
            crates9001.push(line);
            if !crates.push(line) {
                filled = true;
            }
        } else {

            crates.parse_cmd(line);
            crates9001.parse_cmd9001(line);
        }


        
    }
    crates.print();
    crates9001.print();
}
