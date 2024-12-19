use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn part_a(input: &str) -> usize {
    let (ava, towels) = parse(input);
    let pat = ava.join("|");
    let re = Regex::new(format!(r"^({pat})+$").as_str()).unwrap();
    towels.into_iter().filter(|t| re.is_match(t)).count()
}

pub fn part_b(input: &str) -> i64 {
    let (ava, towels) = parse(input);
    towels
        .into_iter()
        .enumerate()
        .map(|(i, t)| {
            println!("Starting {i}");
            let mut lefts = vec![t];
            let mut done = 0;
            let mut cache = HashMap::new();
            while let Some(left) = lefts.pop() {
                get_options(&ava, left).iter().for_each(|&o| match o {
                    None => done += 1,
                    Some(l) => {
                        if l.len() > t.len() / 2 {
                            lefts.push(l);
                            return;
                        }

                        done += *cache
                            .entry(l.to_string())
                            .or_insert_with(|| compute_directly(&ava, l));
                    }
                });
            }
            return done;
        })
        .sum()
}

fn compute_directly(ava: &Vec<&str>, l: &str) -> i64 {
    // If 10 left, just compute and cache
    let mut lefts = vec![l];
    let mut done_10 = 0;
    while let Some(left) = lefts.pop() {
        get_options(&ava, left).iter().for_each(|&o| match o {
            None => done_10 += 1,
            Some(l) => lefts.push(l),
        });
    }
    return done_10;
}

fn get_options<'a>(ava: &'a Vec<&'a str>, left: &'a str) -> Vec<Option<&'a str>> {
    ava.iter()
        .filter(|&a| left.starts_with(a))
        .map(|a| {
            let new_left = &left[a.len()..];
            if new_left.is_empty() {
                None
            } else {
                Some(new_left)
            }
        })
        .collect_vec()
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
