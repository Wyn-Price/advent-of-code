use std::collections::HashMap;

const TURN_LEFT: bool = false;
const TURN_RIGHT: bool = true;

type Pos = (isize, isize);
type Dir = (i8, i8);

const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);
const LEFT: Dir = (-1, 0);
const RIGHT: Dir = (1, 0);

pub fn part_a(input: &str) -> i64 {
    compute(input, false)
}

pub fn part_b(input: &str) -> i64 {
    compute(input, true)
}

fn compute(input: &str, part2: bool) -> i64 {
    let (map, commands) = parse(input);

    let width = *map.keys().map(|(x, _)| x).max().unwrap();
    let height = *map.keys().map(|(_, y)| y).max().unwrap();

    let x_start = *map
        .keys()
        .filter(|(_, y)| *y == 0)
        .map(|(x, _)| x)
        .min()
        .unwrap();

    let mut position: Pos = (x_start, 0);
    let mut dir: Dir = (1, 0);

    let mut cube_face_mappings: HashMap<(Pos, Dir), (Pos, Dir)> = HashMap::new();

    // lol hope your input is the same shape as mine
    // colours -> https://i.imgur.com/fockb7P.png

    //red
    for y in 0..50 {
        let f = (49, y);
        let t = (-1, 100 + 49 - y);

        cube_face_mappings.insert((f, LEFT), (t, RIGHT));
        cube_face_mappings.insert((t, LEFT), (f, RIGHT));
    }

    //yellow
    for y in 0..50 {
        let f = (49, 50 + y);
        let t = (y, 99);

        cube_face_mappings.insert((f, LEFT), (t, DOWN));
        cube_face_mappings.insert((t, UP), (f, RIGHT));
    }

    //green
    for y in 0..50 {
        let f = (50 + y, -1);
        let t = (-1, 150 + y);

        cube_face_mappings.insert((f, UP), (t, RIGHT));
        cube_face_mappings.insert((t, LEFT), (f, DOWN));
    }

    //blue
    for y in 0..50 {
        let f = (y, 200);
        let t = (100 + y, -1);

        cube_face_mappings.insert((f, DOWN), (t, DOWN));
        cube_face_mappings.insert((t, UP), (f, UP));
    }

    //purple
    for y in 0..50 {
        let f = (100 + y, 50);
        let t = (100, 50 + y);

        cube_face_mappings.insert((f, DOWN), (t, LEFT));
        cube_face_mappings.insert((t, RIGHT), (f, UP));
    }

    //white
    for y in 0..50 {
        let f = (150, y);
        let t = (100, 100 + 49 - y);

        cube_face_mappings.insert((f, RIGHT), (t, LEFT));
        cube_face_mappings.insert((t, RIGHT), (f, LEFT));
    }

    //black
    for y in 0..50 {
        let f = (50 + y, 150);
        let t = (50, 150 + y);

        cube_face_mappings.insert((f, DOWN), (t, LEFT));
        cube_face_mappings.insert((t, RIGHT), (f, UP));
    }

    for cmd in commands {
        match cmd {
            Command::Turn(right) => {
                //-1, 0 -> 0, 1 -> 1, 0 -> 0, -1
                let new_dir: Dir = if right {
                    (-dir.1, dir.0)
                } else {
                    (dir.1, -dir.0)
                };
                dir = new_dir;
            }
            Command::Move(amount) => {
                for _ in 0..amount {
                    let mut new_pos: Pos =
                        ((position.0 + dir.0 as isize), (position.1 + dir.1 as isize));

                    let mut new_dir = dir;

                    //Wrap the position around
                    if !map.contains_key(&new_pos) {
                        if part2 {
                            let (wrapped_pos, next_dir) =
                                cube_face_mappings.get(&(new_pos, dir)).unwrap();

                            new_pos = (
                                wrapped_pos.0 + next_dir.0 as isize,
                                wrapped_pos.1 + next_dir.1 as isize,
                            );

                            new_dir = (next_dir.0, next_dir.1);
                        } else {
                            let mut wrapped_starting_pos: Pos = match dir {
                                (0, 1) => (position.0, 0),
                                (0, -1) => (position.0, height),
                                (1, 0) => (0, position.1),
                                (-1, 0) => (width, position.1),
                                _ => panic!("Unknown dir {dir:?}"),
                            };
                            while !map.contains_key(&wrapped_starting_pos) {
                                wrapped_starting_pos = (
                                    (wrapped_starting_pos.0 + dir.0 as isize),
                                    (wrapped_starting_pos.1 + dir.1 as isize),
                                );
                            }

                            new_pos = wrapped_starting_pos;
                        }
                    }

                    let wall = map.get(&new_pos).unwrap();
                    if *wall {
                        break;
                    } else {
                        position = new_pos;
                        dir = new_dir;
                    }
                }
            }
        }
    }

    let dir_score = match dir {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!("Unknown dir {dir:?}"),
    };

    1000 * (position.1 + 1) as i64 + 4 * (position.0 + 1) as i64 + dir_score
}

fn parse(input: &str) -> (HashMap<Pos, bool>, Vec<Command>) {
    let (game, command_string) = input.split_once("\n\n").unwrap();
    let map = game
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .map(move |(x, c)| match c {
                    '.' => ((x as isize, y as isize), false),
                    '#' => ((x as isize, y as isize), true),
                    _ => panic!("Don't know how to handle {c}"),
                })
        })
        .collect();

    let mut number = "".to_owned();
    let mut commands = vec![];
    for c in command_string.trim().chars() {
        if c >= '0' && c <= '9' {
            number += c.to_string().as_str();
            continue;
        }

        commands.push(Command::Move(number.parse().unwrap()));
        number = "".to_owned();

        match c {
            'L' => commands.push(Command::Turn(TURN_LEFT)),
            'R' => commands.push(Command::Turn(TURN_RIGHT)),
            _ => panic!("Unknown char {c}"),
        }
    }
    if number != "" {
        commands.push(Command::Move(number.parse().unwrap()));
    }

    (map, commands)
}

#[derive(Debug)]
enum Command {
    Move(usize),
    Turn(bool),
}
