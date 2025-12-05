use std::ops::RangeInclusive;

pub fn part_a(input: &str) -> i64 {
    let (ranges, nums) = parse(input);

    nums.into_iter()
        .filter(|x| ranges.iter().any(|range| range.contains(x)))
        .count() as i64
}

pub fn part_b(input: &str) -> i64 {
    let (mut ranges, _) = parse(input);

    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<i64>> = vec![];

    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r.start() <= last.end() {
                let &s = last.start();
                let &e = last.end().max(r.end());
                *last = s..=e;
                continue;
            }
        }
        merged.push(r);
    }

    merged.into_iter().map(|r| r.end() - r.start() + 1).sum()
}

fn parse(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let (ranges, nums) = input.split_once("\n\n").unwrap();

    return (
        ranges
            .lines()
            .into_iter()
            .map(|line| {
                let (l, r) = line.split_once("-").unwrap();
                return l.parse::<i64>().unwrap()..=r.parse::<i64>().unwrap();
            })
            .collect(),
        nums.lines()
            .into_iter()
            .map(|l| l.parse().unwrap())
            .collect(),
    );
}
