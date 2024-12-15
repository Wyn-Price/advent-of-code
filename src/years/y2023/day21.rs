use std::collections::HashSet;

type Board = Vec<Vec<(usize, usize, char)>>;
pub fn part_a(input: &str) -> i64 {
    let board = parse(input);
    let &(px, py, _) = board.iter().flatten().find(|(_, _, c)| *c == 'S').unwrap();

    let mut next_queue = vec![(px as isize, py as isize)];

    let height = board.len() as isize;
    let width = board[0].len() as isize;

    for _ in 0..64 {
        let queue = next_queue;
        let mut reached: HashSet<(isize, isize)> = HashSet::from_iter(queue.iter().copied());
        next_queue = vec![];
        for (x, y) in queue {
            for (dx, dy) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx >= width || ny >= height {
                    continue;
                }
                if board[ny as usize][nx as usize].2 != '#' && !reached.contains(&(nx, ny)) {
                    reached.insert((nx, ny));
                    next_queue.push((nx, ny));
                }
            }
        }
    }

    next_queue.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let board = parse(input);
    let &(px, py, _) = board.iter().flatten().find(|(_, _, c)| *c == 'S').unwrap();
    let mut next_queue = vec![(px as isize, py as isize)];

    // Assuming a square
    let size = board.len() as isize;

    let steps = 26501365;
    let off = steps % size;

    let mut sizes = vec![];

    for i in 0..(off + size * 2) {
        let queue = next_queue;
        let mut reached: HashSet<(isize, isize)> = HashSet::from_iter(queue.iter().copied());
        next_queue = vec![];
        for (x, y) in queue {
            for (dx, dy) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                let mx = nx.rem_euclid(size);
                let my = ny.rem_euclid(size);

                if board[my as usize][mx as usize].2 != '#' && !reached.contains(&(nx, ny)) {
                    reached.insert((nx, ny));
                    next_queue.push((nx, ny));
                }
            }
        }
        if (i + 1) % size == off {
            sizes.push(next_queue.len() as i64);
        }
    }

    // Lagrange's Interpolation

    let y0 = sizes[0];
    let y1 = sizes[1];
    let y2 = sizes[2];

    // x0 = 0, x1 = 1, x2 = 2
    let a = y0 / 2 - y1 + y2 / 2;
    let b = -3 * (y0 / 2) + 2 * y1 - y2 / 2;
    let c = y0;

    let x = ((steps - off) / size) as i64;
    return a * x * x + b * x + c;
}

fn parse(input: &str) -> Board {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| line.char_indices().map(|(x, c)| (x, y, c)).collect())
        .collect()
}
