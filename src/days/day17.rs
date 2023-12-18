use core::panic;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Dir {
    None,
    Up,
    Right,
    Down,
    Left,
}

type Board = Vec<Vec<u32>>;

pub fn part_a(input: &str) -> i64 {
    let board = parse(input);
    dijkstra(&board, 1, 3)
}

pub fn part_b(input: &str) -> i64 {
    let board = parse(input);
    dijkstra(&board, 4, 10)
}

impl Dir {
    fn turn90(&self) -> Vec<Dir> {
        match self {
            Dir::None => vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right],
            Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
            Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
        }
    }
}

fn dijkstra(board: &Board, min: i64, max: i64) -> i64 {
    let height = board.len() as i64;
    let width = board[0].len() as i64;
    let mut dists = HashMap::new();
    let mut queue = vec![(0, (0, 0, Dir::None))];

    while let Some((cost, node)) = queue.pop() {
        let (x, y, c_dir) = node;

        if x == width - 1 && y == height - 1 {
            return cost;
        }

        if let Some(&c) = dists.get(&node) {
            // Couldn't we just propagate the change?
            if cost > c {
                continue;
            }
        }

        for dir in c_dir.turn90() {
            let mut sum = cost;
            for dist in 1..max + 1 {
                let nx = match dir {
                    Dir::Left => x - dist,
                    Dir::Right => x + dist,
                    _ => x,
                };

                let ny = match dir {
                    Dir::Up => y - dist,
                    Dir::Down => y + dist,
                    _ => y,
                };

                if nx < 0 || nx >= width || ny < 0 || ny >= height {
                    continue;
                }

                sum += board[ny as usize][nx as usize] as i64;

                let node = (nx, ny, dir);
                let cd = dists.get(&node);
                if min <= dist && (cd.is_none() || sum < *cd.unwrap()) {
                    dists.insert(node, sum);
                    queue.push((sum, node))
                }
            }
        }
        // We want to pop the smallest next, so sort the list largest to smallest
        queue.sort_by(|&a, &b| b.0.cmp(&a.0))
    }
    panic!();
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}
