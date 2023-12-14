use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
};

use itertools::{Either, Itertools};

struct Board {
    width: i32,
    height: i32,
    cubes: Vec<(i32, i32)>,
    rounded: Vec<(i32, i32)>,
}
pub fn part_a(input: &str) -> i64 {
    let mut board = parse(input);
    tilt(&mut board, 0, -1);
    score(&board)
}

pub fn part_b(input: &str) -> i64 {
    let mut board = parse(input);
    let mut cache: HashMap<Vec<(i32, i32)>, i32> = HashMap::new();
    let total = 1_000_000_000;
    for i in 0..total {
        tilt(&mut board, 0, -1);
        tilt(&mut board, -1, 0);
        tilt(&mut board, 0, 1);
        tilt(&mut board, 1, 0);

        let mut key = board.rounded.clone();
        key.sort();

        if let Some(&loop_start) = cache.get(&key) {
            let loop_size = i - loop_start;
            let left = total - i - 1;
            println!("Loop detected {} -> {}", loop_start, i);

            let over = left % loop_size;

            for _ in 0..over {
                tilt(&mut board, 0, -1);
                tilt(&mut board, -1, 0);
                tilt(&mut board, 0, 1);
                tilt(&mut board, 1, 0);
            }

            break;
        }
        cache.insert(key, i);
    }
    score(&board)
}

fn score(board: &Board) -> i64 {
    board
        .rounded
        .iter()
        .map(|(_, y)| (board.height + 1 - y) as i64)
        .sum::<i64>()
}

fn tilt(board: &mut Board, dx: i32, dy: i32) {
    let sorted = board
        .rounded
        .iter()
        .enumerate()
        .sorted_by_cached_key(|(_, &(x, y))| if dx == 0 { y * -dy } else { x * -dx })
        .map(|(idx, _)| idx)
        .collect_vec();

    for idx in sorted {
        loop {
            let (x, y) = board.rounded[idx];
            let nx = x + dx;
            let ny = y + dy;

            if board.cubes.contains(&(nx, ny))
                || board.rounded.contains(&(nx, ny))
                || nx < 0
                || ny < 0
                || nx > board.width
                || ny > board.height
            {
                break;
            }

            board.rounded[idx] = (nx, ny);
        }
    }
}

fn parse(input: &str) -> Board {
    let (cubes, rounded): (Vec<_>, Vec<_>) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .filter(|&(_, _, c)| c == 'O' || c == '#')
        .partition_map(|(x, y, c)| {
            if c == 'O' {
                Either::Right((x, y))
            } else {
                Either::Left((x, y))
            }
        });

    let &width = cubes.iter().map(|(x, _)| x).max().unwrap();
    let &height = cubes.iter().map(|(_, y)| y).max().unwrap();
    Board {
        rounded,
        cubes,
        width,
        height,
    }
}
