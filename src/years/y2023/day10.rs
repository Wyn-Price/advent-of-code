type Board = Vec<Vec<char>>;

pub fn part_a(input: &str) -> i64 {
    let board: Board = parse(input);

    let (start_x, start_y) = get_start(&board);

    let max_steps = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .find_map(|&(sx, sy)| {
            let p = get_loop_points(&board, start_x, start_y, sx, sy)?;
            Some(p.len() as i64)
        })
        .unwrap();

    max_steps / 2
}

pub fn part_b(input: &str) -> i64 {
    let board: Board = parse(input);

    let (start_x, start_y) = get_start(&board);

    let points = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .find_map(|&(sx, sy)| get_loop_points(&board, start_x, start_y, sx, sy))
        .unwrap();

    let simple_board = board
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, &c)| {
                    if points.contains(&(x as i32, y as i32)) {
                        c
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect();

    let mut p_outside = vec![];
    let mut p_inside = vec![];

    let grown_board = grow_board(&simple_board);

    grown_board.iter().enumerate().for_each(|(y, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, &c)| c == '.')
            .for_each(|(x, _)| {
                let p = (x as i32, y as i32);
                if p_outside.contains(&p) || p_inside.contains(&p) {
                    return;
                }

                let (mut computed, inside) =
                    expand_space_outwards(&grown_board, x as i32, y as i32);
                match inside {
                    true => p_inside.append(&mut computed),
                    false => p_outside.append(&mut computed),
                };
            })
    });

    p_inside
        .into_iter()
        .filter(|(x, y)| x % 3 == 1 && y % 3 == 1)
        .count() as i64
}

fn get_start(board: &Board) -> (i32, i32) {
    board
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find(|(_, &c)| c == 'S')
                .map(|(x, _)| (x as i32, y as i32))
        })
        .unwrap()
}

fn get_loop_points(
    board: &Board,
    start_x: i32,
    start_y: i32,
    dir_x: i32,
    dir_y: i32,
) -> Option<Vec<(i32, i32)>> {
    let mut px = start_x;
    let mut py = start_y;

    let mut dx = dir_x;
    let mut dy = dir_y;

    let mut points = vec![(px, py)];
    // Go until we reach the start again, excluding the first loop
    while px != start_x || py != start_y || points.len() == 1 {
        if !can_move(&board, px, py, dx, dy) {
            return None;
        }
        let next = next_dir(&board, px, py, dx, dy);
        px += dx;
        py += dy;
        (dx, dy) = next;

        points.push((px, py));
    }
    Some(points)
}

fn grow_board(board: &Board) -> Board {
    let mut rows = vec![];
    board.iter().for_each(|row| {
        for p in 0..3 {
            let grow_row: Vec<_> = row.iter().flat_map(|&r| grow(r)[p].chars()).collect();
            rows.push(grow_row);
        }
    });

    rows
}

// Each grown 3x3 cell should have the same effect
fn grow(char: char) -> Vec<&'static str> {
    match char {
        '.' => vec!["...", "...", "..."],
        'S' => vec!["SSS", "SSS", "SSS"],
        '|' => vec![".|.", ".|.", ".|."],
        '-' => vec!["...", "---", "..."],
        '7' => vec!["...", "-7.", ".|."],
        'J' => vec![".|.", "-J.", "..."],
        'F' => vec!["...", ".F-", ".|."],
        'L' => vec![".|.", ".L-", "..."],
        _ => panic!("Unknown {char}"),
    }
}

fn expand_space_outwards(board: &Board, px: i32, py: i32) -> (Vec<(i32, i32)>, bool) {
    let mut computed = vec![];

    let height = board.len() as i32;
    let width = board[0].len() as i32;

    let mut to_process = vec![(px, py)];

    let mut inside = true;
    while let Some((px, py)) = to_process.pop() {
        computed.push((px, py));
        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let npx = px + dx;
            let npy = py + dy;

            if npx < 0 || npx >= width || npy < 0 || npy >= height {
                inside = false;
                continue;
            }

            if computed.contains(&(npx, npy)) || to_process.contains(&(npx, npy)) {
                continue;
            }

            let c = board[npy as usize][npx as usize];
            if c != '.' {
                continue;
            }
            to_process.push((npx, npy));
        }
    }

    return (computed, inside);
}

fn can_move(board: &Board, px: i32, py: i32, dx: i32, dy: i32) -> bool {
    if (px == 0 && dx == -1) || (py == 0 && dy == -1) {
        return false;
    }
    let char = board[(py + dy) as usize][(px + dx) as usize];
    match char {
        'S' => true,
        '-' => dx != 0,
        '|' => dy != 0,
        '7' => dx == 1 || dy == -1,
        'J' => dx == 1 || dy == 1,
        'F' => dx == -1 || dy == -1,
        'L' => dx == -1 || dy == 1,
        _ => false,
    }
}

fn next_dir(board: &Board, px: i32, py: i32, dx: i32, dy: i32) -> (i32, i32) {
    let char = board[(py + dy) as usize][(px + dx) as usize];
    match char {
        'S' => (0, 0),
        '-' if dx != 0 => (dx, 0),
        '|' if dy != 0 => (0, dy),
        // (1,0) -> (0,1)
        // (0,-1) -> (-1,0)
        '7' if dx == 1 || dy == -1 => (dy, dx),

        // (1,0) -> (0,-1)
        // (0,1) -> (-1,0)
        'J' if dx == 1 || dy == 1 => (-dy, -dx),

        // (-1,0) -> (0,-1)
        // (0,-1) -> (1,0)
        'F' if dx == -1 || dy == -1 => (-dy, -dx),

        // (0,1) -> (1,0)
        // (-1,0) -> (0,-1)
        'L' if dx == -1 || dy == 1 => (dy, dx),
        _ => panic!("Reached"),
    }
}

fn parse(input: &str) -> Board {
    input.lines().map(|l| l.chars().collect()).collect()
}
