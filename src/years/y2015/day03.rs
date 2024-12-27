use std::collections::HashSet;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let movements = parse(input);
    let mut pos = (0,0);
    let mut visited = HashSet::new();
    visited.insert(pos);

    for (mx, my) in movements {
        pos = (pos.0 + mx, pos.1 + my);
        visited.insert(pos);
    }

    visited.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let movements = parse(input);
    let mut pos = (0,0);
    let mut rpos = (0,0);
    let mut visited = HashSet::new();
    visited.insert(pos);

    for ((mx, my), (rmx, rmy)) in movements.into_iter().tuples(){
        pos = (pos.0 + mx, pos.1 + my);
        rpos = (rpos.0 + rmx, rpos.1 + rmy);
        visited.insert(pos);
        visited.insert(rpos);
    }

    visited.len() as i64
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    input.trim().chars().map(|c| match c {
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        '^' => (-1, 0),
        _ => panic!("Unknown char {c}")
    }).collect_vec()
}
