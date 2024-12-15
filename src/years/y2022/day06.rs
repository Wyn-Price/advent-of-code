use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    parse::<4>(input)
}

pub fn part_b(input: &str) -> i64 {
    parse::<14>(input)
}

fn parse<const N: usize>(input: &str) -> i64 {
    input
        .chars()
        .into_iter()
        .collect::<Vec<_>>()
        .array_windows::<N>()
        .copied()
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == N)
        .unwrap()
        .0 as i64
        + N as i64
}
