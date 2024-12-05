use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (ordering, updates) = parse(input);

    updates
        .into_iter()
        .filter(|up| {
            up.iter().enumerate().all(|(id, ele)| {
                up.iter()
                    .skip(id + 1)
                    .all(|other| ordering.contains(&(*ele, *other)))
            })
        })
        .map(|l| l[(l.len() - 1) / 2] as i64)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let (ordering, updates) = parse(input);

    updates
        .into_iter()
        .filter(|up| {
            !up.iter().enumerate().all(|(id, ele)| {
                up.iter()
                    .skip(id + 1)
                    .all(|other| ordering.contains(&(*ele, *other)))
            })
        })
        .map(|mut l| {
            l.sort_by(|a, b| {
                if ordering.contains(&(*a, *b)) {
                    return Ordering::Less;
                }
                if ordering.contains(&(*b, *a)) {
                    return Ordering::Greater;
                }
                panic!("No ordering for {a} {b}")
            });
            l
        })
        .map(|l| l[(l.len() - 1) / 2] as i64)
        .sum()
}

fn parse(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let (ordering, updates) = input.split("\n\n").collect_tuple().unwrap();

    return (
        ordering
            .lines()
            .map(|l| {
                l.split("|")
                    .map(|p| p.parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        updates
            .lines()
            .map(|l| l.split(",").map(|p| p.parse().unwrap()).collect_vec())
            .collect_vec(),
    );
}
