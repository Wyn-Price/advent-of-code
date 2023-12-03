use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let numbers = parse(input);
    locations(input)
        .into_iter()
        .map(|(x, y, _)| {
            numbers
                .iter()
                .filter(|(xs, xe, ys, _)| {
                    x >= *xs - 1 && x <= *xe + 1 && y >= ys - 1 && y <= ys + 1
                })
                .map(|(_, _, _, num)| *num)
                .sum::<i32>() as i64
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let numbers = parse(input);
    locations(input)
        .into_iter()
        .filter(|&(_, _, c)| c == "*")
        .filter_map(|(x, y, _)| {
            let vec: Vec<_> = numbers
                .iter()
                .filter(|(xs, xe, ys, _)| {
                    x >= *xs - 1 && x <= *xe + 1 && y >= ys - 1 && y <= ys + 1
                })
                .map(|(_, _, _, num)| *num)
                .collect();

            if vec.len() != 2 {
                return None;
            }

            return Some(vec[0] as i64 * vec[1] as i64);
        })
        .sum()
}

fn parse(input: &str) -> Vec<(isize, isize, isize, i32)> {
    let regex = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            regex.captures_iter(line).map(move |caps| {
                let m = caps.get(0).unwrap();
                return (
                    m.start() as isize,
                    m.end() as isize - 1,
                    y as isize,
                    m.as_str().parse::<i32>().unwrap(),
                );
            })
        })
        .collect()
}

fn locations(input: &str) -> Vec<(isize, isize, &str)> {
    let regex = Regex::new(r"[^.\d]").unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            regex.captures_iter(line).map(move |caps| {
                let m = caps.get(0).unwrap();
                return (m.start() as isize, y as isize, m.as_str());
            })
        })
        .collect()
}
