use std::collections::HashSet;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let grid = parse(input);
    solve(grid, true)
}

pub fn part_b(input: &str) -> i64 {
    let grid = parse(input);
    solve(grid, false)
}

fn solve(grid: Vec<Vec<i32>>, single_path: bool) -> i64 {
    grid.iter()
        .enumerate()
        .flat_map(|(y, ys)| {
            ys.iter()
                .enumerate()
                .filter(|&(_, v)| *v == 0)
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .map(|starting| {
            let mut been = HashSet::new();

            let mut counter = 0;

            let mut heads = vec![starting];

            while !heads.is_empty() {
                let (hx, hy) = heads.pop().unwrap();
                let hv = grid[hy as usize][hx as usize];
                been.insert((hx, hy));
                if hv == 9 {
                    counter += 1;
                    continue;
                }

                for (dx, dy) in vec![(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    let nx = hx + dx;
                    let ny = hy + dy;

                    if single_path && been.contains(&(nx, ny)) {
                        continue;
                    }

                    if nx < 0 || ny < 0 {
                        continue;
                    }

                    if let Some(v) = grid
                        .get(ny as usize)
                        .map(|ys| ys.get(nx as usize))
                        .flatten()
                    {
                        if v - hv == 1 {
                            heads.push((nx, ny));
                        }
                    }
                }
            }

            counter
        })
        .sum()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect()
}
