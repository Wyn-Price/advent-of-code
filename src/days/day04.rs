use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|(winning, have)| {
            let c = winning
                .iter()
                .filter(|&a| have.iter().any(|&b| *a == b))
                .count() as u32;
            if c == 0 {
                0_i64
            } else {
                2_i64.pow(c - 1)
            }
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let scores: Vec<_> = parse(input)
        .into_iter()
        .map(|(winning, have)| {
            winning
                .iter()
                .filter(|&a| have.iter().any(|&b| *a == b))
                .count()
        })
        .collect();

    let mut counters = vec![1_usize; scores.len()];

    for i in 0..scores.len() {
        let score = scores[i];
        let counter = counters[i];
        for s in 0..score {
            counters[i + s + 1] += counter;
        }
    }

    counters.into_iter().sum::<usize>() as i64
}

fn parse(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    let regex = Regex::new(r"Card\s+\d+: ([\s\d]+)\|([\s\d]+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let winning = part_nums(captures.get(1).unwrap().as_str());
            let have = part_nums(captures.get(2).unwrap().as_str());
            return (winning, have);
        })
        .collect()
}

fn part_nums(input: &str) -> Vec<u8> {
    input
        .split_ascii_whitespace()
        .into_iter()
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
