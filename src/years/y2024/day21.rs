use std::{collections::HashMap, i64};

use memoize::memoize;

pub fn part_a(input: &str) -> i64 {
    let codes = parse(input);

    codes
        .into_iter()
        .map(|code| {
            let moves = solve(code.to_owned(), 0, 3);
            code[0..code.len() - 1].parse::<i64>().unwrap() * moves
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let codes = parse(input);

    codes
        .into_iter()
        .map(|code| {
            let moves = solve(code.to_owned(), 0, 26);
            code[0..code.len() - 1].parse::<i64>().unwrap() * moves
        })
        .sum()
}

#[memoize]
fn solve(code: String, depth: i32, max_depth: i32) -> i64 {
    if depth == max_depth {
        return code.len() as i64;
    }
    let keypad = if depth == 0 {
        vec!["789", "456", "123", " 0A"]
    } else {
        vec![" ^A", "<v>"]
    };
    let char2pos = keypad
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .map(move |(x, c)| (c, (x as i32, y as i32)))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();

    let (ex, ey) = char2pos[&' '];

    let mut result = 0;

    let mut pos = char2pos[&'A'];
    for c in code.chars() {
        let (ax, ay) = pos;
        let &(px, py) = char2pos.get(&c).unwrap();

        let dx = px - ax;
        let dy = py - ay;

        let sdy = if dy < 0 {
            &"^".repeat(-dy as usize)
        } else if dy > 0 {
            &"v".repeat(dy as usize)
        } else {
            ""
        };

        let sdx = if dx < 0 {
            &"<".repeat(-dx as usize)
        } else if dx > 0 {
            &">".repeat(dx as usize)
        } else {
            ""
        };

        let mut min = i64::MAX;
        // Don't go over the blank spaces
        if px != ex || ay != ey {
            min = min.min(solve(sdx.to_owned() + sdy + "A", depth + 1, max_depth));
        }
        if ax != ex || py != ey {
            min = min.min(solve(sdy.to_owned() + sdx + "A", depth + 1, max_depth));
        }

        result += min;

        pos = (px, py);
    }

    result
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}
