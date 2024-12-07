use std::collections::HashSet;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let mut game = parse(input);

    let mut positions = HashSet::new();

    while game.guard.0 >= 0
        && game.guard.0 < game.bound.0
        && game.guard.1 >= 0
        && game.guard.1 < game.bound.1
    {
        positions.insert(game.guard.clone());

        let mut guard_move = game.guard_dir.move_dir();
        let mut new_pos = (game.guard.0 + guard_move.0, game.guard.1 + guard_move.1);
        while game.obstacles.contains(&new_pos) {
            game.guard_dir = game.guard_dir.rotate_90();
            guard_move = game.guard_dir.move_dir();
            new_pos = (game.guard.0 + guard_move.0, game.guard.1 + guard_move.1);
        }

        game.guard = new_pos;
    }

    positions.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let master_game = parse(input);

    let mut counter = 0;

    for ox in 0..master_game.bound.0 {
        for oy in 0..master_game.bound.1 {
            let oby = (ox, oy);
            if master_game.obstacles.contains(&oby) || master_game.guard == oby {
                continue;
            }

            let mut loops = false;

            let mut game = master_game.clone();
            game.obstacles.insert(oby);

            let mut before = HashSet::new();

            while game.guard.0 >= 0
                && game.guard.0 < game.bound.0
                && game.guard.1 >= 0
                && game.guard.1 < game.bound.1
            {
                let key = (game.guard.clone(), game.guard_dir.clone());
                if !before.insert(key) {
                    loops = true;
                    break;
                }

                let mut guard_move = game.guard_dir.move_dir();
                let mut new_pos = (game.guard.0 + guard_move.0, game.guard.1 + guard_move.1);
                while game.obstacles.contains(&new_pos) {
                    game.guard_dir = game.guard_dir.rotate_90();
                    guard_move = game.guard_dir.move_dir();
                    new_pos = (game.guard.0 + guard_move.0, game.guard.1 + guard_move.1);
                }

                game.guard = new_pos;
            }

            if loops {
                counter += 1;
            }
        }
    }

    counter
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn move_dir(&self) -> (i32, i32) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
        }
    }

    fn rotate_90(&self) -> Dir {
        match self {
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
        }
    }
}

#[derive(Clone)]
struct Game {
    obstacles: HashSet<(i32, i32)>,
    guard: (i32, i32),
    guard_dir: Dir,
    bound: (i32, i32),
}

fn parse(input: &str) -> Game {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut obstacles = HashSet::new();
    let mut guard = None;

    for x in 0..w {
        for y in 0..h {
            let char = grid[y as usize][x as usize];
            match char {
                '#' => {
                    obstacles.insert((x, y));
                }
                '^' => {
                    guard = Some((x, y));
                }
                '.' => {}
                _ => panic!("Unknown {char}"),
            }
        }
    }

    return Game {
        obstacles,
        guard_dir: Dir::Up,
        guard: guard.unwrap(),
        bound: (w, h),
    };
}
