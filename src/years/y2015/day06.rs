use itertools::Itertools;
use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let cmds = parse(input);
    let mut lights = [[false; 1000]; 1000];
    for (cmd, x1, y1, x2, y2) in cmds {
        for x in x1..=x2 {
            for y in y1..=y2 {
                match cmd {
                    Cmd::Toggle => lights[x][y] = !lights[x][y],
                    Cmd::Set(b) => lights[x][y] = b,
                }
            }
        }
    }

    (0..1000)
        .cartesian_product(0..1000)
        .filter(|&(x, y)| lights[x][y])
        .count() as i64
}

pub fn part_b(input: &str) -> i64 {
    let cmds = parse(input);
    let mut lights = [[0; 1000]; 1000];
    for (cmd, x1, y1, x2, y2) in cmds {
        for x in x1..=x2 {
            for y in y1..=y2 {
                match cmd {
                    Cmd::Toggle => lights[x][y] += 2,
                    Cmd::Set(b) => {
                        lights[x][y] += if b { 1 } else { -1 };
                        lights[x][y] = lights[x][y].max(0)
                    }
                }
            }
        }
    }

    (0..1000)
        .cartesian_product(0..1000)
        .map(|(x, y)| lights[x][y])
        .sum()
}

enum Cmd {
    Set(bool),
    Toggle,
}

fn parse(input: &str) -> Vec<(Cmd, usize, usize, usize, usize)> {
    let re = Regex::new(r"(.+) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let (_, cmd, x1, y1, x2, y2) = re
                .captures(l)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().as_str())
                .collect_tuple()
                .unwrap();
            let c = match cmd {
                "turn on" => Cmd::Set(true),
                "turn off" => Cmd::Set(false),
                "toggle" => Cmd::Toggle,
                _ => panic!("Unknown cmd {cmd}"),
            };
            (
                c,
                x1.parse().unwrap(),
                y1.parse().unwrap(),
                x2.parse().unwrap(),
                y2.parse().unwrap(),
            )
        })
        .collect_vec()
}
