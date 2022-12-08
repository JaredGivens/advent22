use std::fs;

struct Job {
    f: i32,
    b: i32,
}

impl Job {
    pub fn new(s: &str) -> Self {
        let d = s.find(|c| c == '-').unwrap();
        Self {
            f: s[..d].parse::<i32>().unwrap(),
            b: s[d + 1..].parse::<i32>().unwrap(),
        }
    }
    pub fn ctns(&self, o: &Job) -> bool {
        self.f <= o.f && self.b >= o.b
    }

}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let dem = line.find(|c| c == ',').unwrap();
        let j0 = Job::new(&line[..dem]);
        let j1 = Job::new(&line[dem + 1..]);
        if j0.ctns(&j1) || j1.ctns(&j0) {
            sum += 1;
        }

    }
    println!("{sum}");
    
    
    println!("Hello, world!");
}
