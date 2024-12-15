use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

pub fn part_a(input: &str) -> i64 {
    let (elves, _) = compute(parse(input), true);

    let min_x = *elves.iter().map(|(x, _)| x).min().unwrap();
    let min_y = *elves.iter().map(|(_, y)| y).min().unwrap();
    let max_x = *elves.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *elves.iter().map(|(_, y)| y).max().unwrap();

    let mut count = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !elves.contains(&(x, y)) {
                count += 1;
            }
        }
    }
    count
}

pub fn part_b(input: &str) -> i64 {
    let (_, round) = compute(parse(input), false);
    round
}

fn compute(mut elves: HashSet<Pos>, part1: bool) -> (HashSet<Pos>, i64) {
    let mut round = 0;
    loop {
        // Why 8? because 10 - 2 = 8 🙂
        if part1 && round == 8 {
            break;
        }
        let mut moved = false;
        // Map of target_pos -> Vec<src_pos>
        let mut new_elves: HashMap<Pos, Vec<Pos>> = HashMap::new();

        for pos in elves.iter() {
            let north = compute_next_pos(pos, &elves, 0, -1, 1, 0);
            let south = compute_next_pos(pos, &elves, 0, 1, 1, 0);
            let west = compute_next_pos(pos, &elves, -1, 0, 0, 1);
            let east = compute_next_pos(pos, &elves, 1, 0, 0, 1);

            let mut directions = vec![north, south, west, east];
            directions.rotate_left(round % 4);

            let should_move = directions.iter().any(|o| o.is_none());

            let new_pos = if should_move {
                moved = true;
                directions[0]
                    .or(directions[1])
                    .or(directions[2])
                    .or(directions[3])
                    .unwrap_or(*pos)
            } else {
                *pos
            };

            new_elves.entry(new_pos).or_insert(vec![]).push(*pos);
        }

        let mut next_elves: HashSet<Pos> = HashSet::new();
        for (target, srcs) in new_elves {
            if srcs.len() == 1 {
                next_elves.insert(target);
            } else {
                for s in srcs {
                    next_elves.insert(s);
                }
            }
        }

        round += 1;

        elves = next_elves;
        if !moved {
            break;
        }
    }

    return (elves, round as i64);
}

fn compute_next_pos(
    pos: &Pos,
    elves: &HashSet<Pos>,
    xd: i32,
    yd: i32,
    xm: i32,
    ym: i32,
) -> Option<Pos> {
    let mut valid = true;
    for n in -1..=1 {
        if elves.contains(&(pos.0 + n * xm + xd, pos.1 + n * ym + yd)) {
            valid = false;
            break;
        }
    }
    if valid {
        return Some((pos.0 + xd, pos.1 + yd));
    }
    return None;
}

#[allow(dead_code)]
fn debug_print(elves: &HashSet<Pos>) {
    let min_x = *elves.iter().map(|(x, _)| x).min().unwrap();
    let min_y = *elves.iter().map(|(_, y)| y).min().unwrap();
    let max_x = *elves.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *elves.iter().map(|(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(x, y)) {
                print!("#");
            } else if x == 0 && y == 0 {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse(input: &str) -> HashSet<Pos> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, char)| {
                if char == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}
