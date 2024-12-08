use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (freqs, w, h) = parse(input);

    let mut annodes = HashSet::new();

    for (&(x, y), &c) in freqs.iter() {
        for (&(ox, oy), &oc) in freqs.iter() {
            if c != oc || (x == ox && y == oy) {
                continue;
            }
            let dx = ox as i32 - x as i32;
            let dy = oy as i32 - y as i32;

            let nx = x as i32 - dx;
            let ny = y as i32 - dy;
            if ny < 0 || nx < 0 || ny >= h as i32 || nx >= w as i32 {
                continue;
            }

            annodes.insert((nx, ny));
        }
    }

    annodes.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let (freqs, w, h) = parse(input);

    let mut annodes = HashSet::new();

    for (&(x, y), &c) in freqs.iter() {
        for (&(ox, oy), &oc) in freqs.iter() {
            if c != oc || (x == ox && y == oy) {
                continue;
            }
            annodes.insert((x as i32, y as i32));
            let dx = ox as i32 - x as i32;
            let dy = oy as i32 - y as i32;

            let mut nx = x as i32 - dx;
            let mut ny = y as i32 - dy;
            loop {
                if ny < 0 || nx < 0 || ny >= h as i32 || nx >= w as i32 {
                    break;
                }

                annodes.insert((nx, ny));

                nx -= dx;
                ny -= dy;
            }
        }
    }

    annodes.len() as i64
}

fn parse(input: &str) -> (HashMap<(usize, usize), char>, usize, usize) {
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(|(x, c)| ((x, y), c))
                    .collect_vec()
            })
            .collect(),
        input.lines().next().unwrap().len(),
        input.lines().collect_vec().len(),
    )
}
