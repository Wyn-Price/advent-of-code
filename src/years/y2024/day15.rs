use std::collections::HashSet;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (mut board, moves) = parse(input);
    for (mx, my) in moves {
        let mut boxes_to_shift = vec![];
        let mut next = (board.robot.0 + mx, board.robot.1 + my);
        while board.boxes.contains(&next) {
            boxes_to_shift.push(next);
            next = (next.0 + mx, next.1 + my);
        }

        if board.walls.contains(&next) {
            continue;
        }

        board.robot = (board.robot.0 + mx, board.robot.1 + my);

        boxes_to_shift.iter().for_each(|b| {
            board.boxes.remove(b);
        });
        boxes_to_shift
            .iter()
            .map(|b| (b.0 + mx, b.1 + my))
            .for_each(|b| {
                board.boxes.insert(b);
            });
    }

    board
        .boxes
        .iter()
        .map(|b| b.0 + 100 * b.1)
        .map(|s| s as i64)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let (mut board, moves) = parse(input);
    // Boxes are defined as their left position
    board.boxes = board.boxes.into_iter().map(|(x, y)| (x * 2, y)).collect();
    board.walls = board
        .walls
        .into_iter()
        .flat_map(|(x, y)| vec![(x * 2, y), (x * 2 + 1, y)])
        .collect();
    board.robot = (board.robot.0 * 2, board.robot.1);
    board.width = board.width * 2 - 1;
    for (mx, my) in moves {
        println!("{mx} {my}");
        let mut boxes_to_shift = vec![];
        // Next contains all the positions, so 2 per box
        let mut next = vec![(board.robot.0 + mx, board.robot.1 + my)];
        let hit_wall: bool = 'main: loop {
            let mut next_next = vec![];
            for (nx, ny) in next {
                if board.walls.contains(&(nx, ny)) {
                    break 'main true;
                }
                let box_next = if board.boxes.contains(&(nx, ny)) {
                    (nx, ny)
                } else if board.boxes.contains(&(nx - 1, ny)) {
                    (nx - 1, ny)
                } else {
                    continue;
                };
                boxes_to_shift.push(box_next);
                // The next position to look based on if we're moving left, right, or vertical
                match mx {
                    0 => {
                        next_next.push((box_next.0 + mx, box_next.1 + my));
                        next_next.push((box_next.0 + mx + 1, box_next.1 + my));
                    }
                    1 => {
                        next_next.push((box_next.0 + mx + 1, box_next.1 + my));
                    }
                    -1 => {
                        next_next.push((box_next.0 + mx, box_next.1 + my));
                    }
                    _ => panic!("Unknown move {mx}"),
                }
            }
            if next_next.is_empty() {
                break false;
            }
            next = next_next;
        };

        if hit_wall {
            continue;
        }

        board.robot = (board.robot.0 + mx, board.robot.1 + my);

        boxes_to_shift.iter().for_each(|b| {
            board.boxes.remove(b);
        });
        boxes_to_shift
            .iter()
            .map(|b| (b.0 + mx, b.1 + my))
            .for_each(|b| {
                board.boxes.insert(b);
            });
    }

    board
        .boxes
        .iter()
        .map(|b| b.0 + 100 * b.1)
        .map(|s| s as i64)
        .sum()
}

#[allow(dead_code)]
fn debug_print(board: &Board, p2: bool) {
    for y in 0..=board.height {
        for x in 0..=board.width {
            let p = (x, y);
            if board.boxes.contains(&p) {
                print!("{}", if p2 { "[" } else { "O" })
            } else if p2 && board.boxes.contains(&(x - 1, y)) {
                print!("]")
            } else if board.walls.contains(&p) {
                print!("#")
            } else if board.robot == p {
                print!("@")
            } else {
                print!(".")
            }
        }
        println!("")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    width: i32,
    height: i32,
    robot: (i32, i32),
    boxes: HashSet<(i32, i32)>,
    walls: HashSet<(i32, i32)>,
}

fn parse(input: &str) -> (Board, Vec<(i32, i32)>) {
    let (board_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut board = Board {
        width: 0,
        height: 0,
        robot: (0, 0),
        boxes: HashSet::new(),
        walls: HashSet::new(),
    };
    board_str.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let p = (x as i32, y as i32);
            match c {
                '#' => {
                    board.width = board.width.max(p.0 + 1);
                    board.height = board.height.max(p.1 + 1);
                    board.walls.insert(p);
                }
                'O' => {
                    board.boxes.insert(p);
                }
                '@' => board.robot = p,
                '.' => {}
                _ => panic!("Unknown char {c} at {x} {y}"),
            }
        })
    });

    let moves = moves_str
        .trim()
        .lines()
        .join("")
        .chars()
        .map(|c| match c {
            '^' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            _ => panic!("Unknown move {c}"),
        })
        .collect_vec();

    return (board, moves);
}
