use itertools::Itertools;

pub fn part_a(input: &str) -> usize {
    let bools = parse(input);
    let up = bools.iter().filter(|b| **b).count();
    let down = bools.iter().filter(|b| !**b).count();
    return up - down;
}

pub fn part_b(input: &str) -> usize {
    let mut f = 0;
    for (i, c) in input.char_indices() {
        f += if c == '(' { 1 } else { -1 };
        if f == -1 {
            return i + 1;
        }
    }
    panic!("")
}

fn parse(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '(').collect_vec()
}
