use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (mut l1, mut l2) = parse(input);
    l1.sort();
    l2.sort();

    l1.into_iter()
        .enumerate()
        .map(|(i, e1)| e1.abs_diff(*l2.get(i).unwrap()) as i64)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let (l1, l2) = parse(input);
    l1.into_iter()
        .map(|e1| e1 as i64 * l2.iter().filter(|e2| **e2 == e1).count() as i64)
        .sum()
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .into_iter()
        .map(|s| {
            s.split_whitespace()
                .map(|p| p.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .unzip()
}
