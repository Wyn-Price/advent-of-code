use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

type Commands = Vec<(Dir, i64)>;

pub fn part_a(input: &str) -> i64 {
    let commands = parse(input, false);
    compute(commands)
}

pub fn part_b(input: &str) -> i64 {
    let commands = parse(input, true);
    compute(commands)
}

fn compute(commands: Commands) -> i64 {
    let mut verticies = vec![(0, 0)];
    let mut position = (0, 0);
    for (dir, am) in commands {
        let (x, y) = position;
        position = match dir {
            Dir::Down => (x, y + am),
            Dir::Up => (x, y - am),
            Dir::Left => (x - am, y),
            Dir::Right => (x + am, y),
        };
        verticies.push(position);
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula#Triangle_formula
    let area = verticies
        .iter()
        .tuple_windows()
        .fold(0, |mut area, (i, j)| {
            area += i.0 * j.1;
            area -= i.1 * j.0;
            area
        });

    let perimeter = verticies
        .iter()
        .tuple_windows()
        .fold(0, |mut peri, (i, j)| {
            peri += i.0.abs_diff(j.0) as i64;
            peri += i.1.abs_diff(j.1) as i64;
            peri
        });

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    area / 2 + perimeter / 2 + 1
}

fn parse(input: &str, use_colour: bool) -> Commands {
    let regex = Regex::new(r"(.) (\d+) \(#(.{5})(.)\)").unwrap();

    input
        .lines()
        .map(|l| {
            let captures = regex.captures(l).unwrap();
            if use_colour {
                let distance = i64::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap();
                let dir = match captures.get(4).unwrap().as_str() {
                    "0" => Dir::Right,
                    "1" => Dir::Down,
                    "2" => Dir::Left,
                    "3" => Dir::Up,
                    _ => panic!("Unknown"),
                };

                (dir, distance)
            } else {
                let dir = match captures.get(1).unwrap().as_str() {
                    "L" => Dir::Left,
                    "R" => Dir::Right,
                    "U" => Dir::Up,
                    "D" => Dir::Down,
                    _ => panic!("Unknown"),
                };

                let num = captures.get(2).unwrap().as_str().parse().unwrap();
                (dir, num)
            }
        })
        .collect_vec()
}
