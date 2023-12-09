use std::collections::HashMap;

use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let (cmd, map) = parse(input);

    let mut node = "AAA";
    let mut iter = 0;
    while node != "ZZZ" {
        let go_left = cmd.chars().nth(iter % cmd.len()).unwrap() == 'L';

        let &(left, right) = map.get(node).unwrap();

        node = match go_left {
            true => left,
            false => right,
        };

        iter += 1;
    }

    iter as i64
}

pub fn part_b(input: &str) -> i64 {
    let (cmd, map) = parse(input);

    map.keys()
        .filter(|k| k.ends_with("A"))
        .map(|&(mut node)| {
            let mut iter = 0;
            while !node.ends_with("Z") {
                let go_left = cmd.chars().nth(iter % cmd.len()).unwrap() == 'L';

                let &(left, right) = map.get(node).unwrap();

                node = match go_left {
                    true => left,
                    false => right,
                };

                iter += 1;
            }
            iter
        })
        .reduce(|a, b| lcm(a, b))
        .unwrap() as i64
}

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let regex = Regex::new(r"^(.+) = \((.+), (.+)\)$").unwrap();
    let (cmd, data) = input.split_once("\n\n").unwrap();

    let map = data
        .lines()
        .map(|l| {
            let cap = regex.captures(l).unwrap();
            let key = cap.get(1).unwrap().as_str();
            let l = cap.get(2).unwrap().as_str();
            let r = cap.get(3).unwrap().as_str();
            return (key, (l, r));
        })
        .collect::<HashMap<_, _>>();

    (cmd, map)
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
