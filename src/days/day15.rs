use std::collections::HashMap;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let strings = parse(input);
    strings.into_iter().map(|s| hash(s) as i64).sum()
}

pub fn part_b(input: &str) -> i64 {
    let strings = parse(input);
    let mut map: HashMap<u8, Vec<(&str, i64)>> = HashMap::new();

    for str in strings {
        if str.contains("-") {
            let label = &str[0..str.len() - 1];
            let lh = hash(label);
            if let Some(vec) = map.get_mut(&lh) {
                vec.retain(|&(str, _)| str != label);
            }
            continue;
        }

        let (label, value) = str.split_once("=").unwrap();
        let lh = hash(label);
        let vec = map.entry(lh).or_insert_with(|| vec![]);

        let focal = value.parse().unwrap();

        let mut set = false;
        vec.iter_mut().for_each(|pair| {
            if pair.0 == label {
                pair.1 = focal;
                set = true;
            }
        });
        if set {
            continue;
        }
        vec.push((label, focal))
    }

    map.keys()
        .into_iter()
        .map(|&k| {
            map.get(&k)
                .unwrap()
                .iter()
                .enumerate()
                .map(|(idx, &(str, focal))| {
                    println!("{} {} in box {} at {}", str, focal, k, idx);
                    (k as i64 + 1) * (idx as i64 + 1) * focal
                })
                .sum::<i64>()
        })
        .sum()
}

fn hash(input: &str) -> u8 {
    let mut cv = 0;
    for c in input.chars() {
        cv += c as i64;
        cv *= 17;
        cv %= 256;
    }

    cv as u8
}

fn parse(input: &str) -> Vec<&str> {
    input.split(",").map(|s| s.trim()).collect_vec()
}
