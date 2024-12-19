use std::rc::Rc;

use itertools::Itertools;
use memoize::memoize;
use regex::Regex;

pub fn part_a(input: &str) -> usize {
    let (ava, towels) = parse(input);
    let pat = ava.join("|");
    let re = Regex::new(format!(r"^({pat})+$").as_str()).unwrap();
    towels.into_iter().filter(|t| re.is_match(t)).count()
}

pub fn part_b(input: &str) -> i64 {
    let (ava, towels) = parse(input);
    let available = Rc::new(ava.into_iter().map(|a| a.to_string()).collect_vec());

    towels
        .into_iter()
        .enumerate()
        .map(|(i, t)| {
            println!("Starting {i}");
            return count(available.clone(), t.to_string());
        })
        .sum()
}

#[memoize]
fn count(available: Rc<Vec<String>>, towel: String) -> i64 {
    if towel.is_empty() {
        return 1;
    }
    return available
        .iter()
        .filter(|&p| towel.starts_with(p))
        .map(|p| count(available.clone(), towel[p.len()..].to_string()))
        .sum();
}

// #[derive(Debug)]
// enum Colour {
//     White,
//     Blue,
//     Black,
//     Red,
//     Green,
// }
// type Towel = Vec<Colour>;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (avaliable, towels) = input.split_once("\n\n").unwrap();
    return (
        avaliable.split(", ").collect_vec(),
        towels.lines().collect_vec(),
    );
}
