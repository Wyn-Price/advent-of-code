use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

type Board = Vec<Vec<char>>;
type Beam = (Dir, i32, i32);

pub fn part_a(input: &str) -> i64 {
    let board = parse(input);
    compute(&board, (Dir::Right, -1, 0))
}

pub fn part_b(input: &str) -> i64 {
    let board = parse(input);

    let width = board[0].len() as i32;
    let height = board.len() as i32;
    (0..width)
        .flat_map(|x| vec![(Dir::Down, x, -1), (Dir::Up, x, height)])
        .chain((0..height).flat_map(|y| vec![(Dir::Right, -1, y), (Dir::Left, width, y)]))
        .map(|beam| compute(&board, beam))
        .max()
        .unwrap()
}

fn compute(board: &Board, starting: Beam) -> i64 {
    let mut beams = vec![starting];

    let mut hit: Vec<Beam> = vec![];
    while !beams.is_empty() {
        beams = beams
            .into_iter()
            .flat_map(|b| move_beam(&board, b))
            .filter(|b| !hit.contains(b))
            .collect();

        for &ele in &beams {
            hit.push(ele)
        }
    }

    hit.into_iter()
        .map(|(_, x, y)| (x, y))
        .unique()
        .collect_vec()
        .len() as i64
}

fn move_beam(board: &Board, (dir, x, y): Beam) -> Vec<Beam> {
    let width = board[0].len() as i32;
    let height = board.len() as i32;

    let dx = match dir {
        Dir::Left => -1,
        Dir::Right => 1,
        _ => 0,
    };

    let dy = match dir {
        Dir::Up => -1,
        Dir::Down => 1,
        _ => 0,
    };

    let nx = x + dx;
    let ny = y + dy;

    if nx < 0 || nx >= width || ny < 0 || ny >= height {
        return vec![];
    }

    let next_char = board[ny as usize][nx as usize];

    let out = match next_char {
        '.' => vec![dir],
        '/' => {
            vec![match dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Up,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Down,
            }]
        }
        '\\' => {
            vec![match dir {
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Up,
            }]
        }
        '|' => match dir {
            Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
            _ => vec![dir],
        },
        '-' => match dir {
            Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
            _ => vec![dir],
        },
        _ => panic!("Unknown char {next_char}"),
    };

    out.into_iter().map(|dir| (dir, nx, ny)).collect_vec()
}

fn parse(input: &str) -> Board {
    input.lines().map(|l| l.chars().collect()).collect()
}
