use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .filter(|&[f1, t1, f2, t2]| (f1 <= f2 && t1 >= t2) || (f2 <= f1 && t2 >= t1))
        .count()
        .try_into()
        .unwrap()
}

pub fn part_b(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .filter(|&[f1, t1, f2, t2]| {
            (f1 <= f2 && t1 >= t2)
                || (f2 <= f1 && t2 >= t1)
                || (t1 >= f2 && t1 <= t2)
                || (f1 >= f2 && f1 <= t2)
        })
        .count()
        .try_into()
        .unwrap()
}

fn parse(input: &str) -> Vec<[usize; 4]> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let c = re.captures(l).unwrap();
            [
                c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                c.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                c.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            ]
        })
        .collect()
}
