pub fn part_a(input: &str) -> i64 {
    parse(input).into_iter().max().unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let mut vec = parse(input);
    vec.sort();
    vec[vec.len() - 3..].iter().sum()
}

fn parse(input: &str) -> Vec<i64> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|l| l.parse::<i64>().unwrap()).sum())
        .collect()
}
