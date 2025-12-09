use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let tiles = parse(input);
    tiles
        .into_iter()
        .combinations(2)
        .map(|ts| {
            let a = ts[0];
            let b = ts[1];

            return (a.0 - b.0 + 1).abs() * (a.1 - b.1 + 1).abs();
        })
        .max()
        .unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let tiles = parse(input);

    let mut xs: Vec<i64> = tiles.iter().map(|t| t.0).collect();
    let mut ys: Vec<i64> = tiles.iter().map(|t| t.1).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let x_map: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_map: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    let compressed_tiles: Vec<(usize, usize)> =
        tiles.iter().map(|&(x, y)| (x_map[&x], y_map[&y])).collect();

    let mut walls = HashSet::new();
    for (a, b) in compressed_tiles.iter().circular_tuple_windows() {
        let (x1, y1) = *a;
        let (x2, y2) = *b;
        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                walls.insert((x as i64, y as i64));
            }
        }
    }

    let w = xs.len() as i64;
    let h = ys.len() as i64;

    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut outside: HashSet<(i64, i64)> = HashSet::new();

    queue.push_back((-1, -1));
    outside.insert((-1, -1));

    while let Some((x, y)) = queue.pop_front() {
        for (nx, ny) in [
            (x + 1, y),
            (x.saturating_sub(1), y),
            (x, y + 1),
            (x, y.saturating_sub(1)),
        ] {
            if nx < -1 || nx > w + 1 || ny < -1 || ny > h + 1 {
                continue;
            }
            if walls.contains(&(nx, ny)) || outside.contains(&(nx, ny)) {
                continue;
            }
            outside.insert((nx, ny));
            queue.push_back((nx, ny));
        }
    }

    compressed_tiles
        .into_iter()
        .combinations(2)
        .filter_map(|ts| {
            let a = ts[0];
            let b = ts[1];

            for (x, y) in (a.0.min(b.0)..=a.0.max(b.0))
                .flat_map(|x| vec![(x, a.1.min(b.1)), (x, a.1.max(b.1))])
                .chain(
                    (a.1.min(b.1)..=a.1.max(b.1))
                        .flat_map(|y| vec![(a.0.min(b.0), y), (a.0.max(b.0), y)]),
                )
            {
                let is_inside = walls.contains(&(x as i64, y as i64))
                    || !outside.contains(&(x as i64, y as i64));
                if !is_inside {
                    return None;
                }
            }

            // Recover original coordinates to compute area
            let area = (xs[a.0].abs_diff(xs[b.0]) + 1) * (ys[a.1].abs_diff(ys[b.1]) + 1);
            Some(area as i64)
        })
        .max()
        .unwrap()
}

type Tile = (i64, i64);

fn parse(input: &str) -> Vec<Tile> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}
