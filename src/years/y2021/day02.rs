use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let mut sub = Submarine {
        depth: 0,
        position: 0,
        aim: 0,
        part_a: true,
    };
    input.lines().for_each(|l| sub.command(l));

    sub.depth * sub.position
}

pub fn part_b(input: &str) -> i64 {
    let mut sub = Submarine {
        depth: 0,
        position: 0,
        aim: 0,
        part_a: false,
    };
    input.lines().for_each(|l| sub.command(l));

    sub.depth * sub.position
}

struct Submarine {
    depth: i64,
    position: i64,
    aim: i64,
    part_a: bool,
}

impl Submarine {
    fn command(&mut self, str: &str) {
        let captures = Regex::new(r"(forward|down|up) (\d+)")
            .unwrap()
            .captures(str)
            .unwrap();

        let command = captures.get(1).unwrap().as_str();
        let amount = captures.get(2).unwrap().as_str().parse().unwrap();

        match command {
            "forward" => self.forward(amount),
            "up" => self.down(-amount),
            "down" => self.down(amount),
            _ => panic!("Unknown input {command}"),
        };
    }

    fn forward(&mut self, x: i64) {
        self.position += x;
        if !self.part_a {
            self.depth += self.aim * x;
        }
    }

    fn down(&mut self, d: i64) {
        if self.part_a {
            self.depth += d;
        } else {
            self.aim += d;
        }
    }
}
