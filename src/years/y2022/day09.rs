use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    compute_rope::<2>(parse(input))
}

pub fn part_b(input: &str) -> i64 {
    compute_rope::<10>(parse(input))
}

fn compute_rope<const N: usize>(input: Vec<(i8, i8, u8)>) -> i64 {
    let mut knots = [(0, 0); N];
    let mut tail_positions = HashSet::new();

    for (x, y, amount) in input {
        for _ in 0..amount {
            knots[0].0 = knots[0].0 + x as i64;
            knots[0].1 = knots[0].1 + y as i64;

            for i in 0..N - 1 {
                let head = knots[i];
                let tail = knots[i + 1];

                let (hx, hy) = head;
                let (tx, ty) = tail;

                let dx = hx.abs_diff(tx);
                let dy = hy.abs_diff(ty);
                let sx = (hx - tx).signum();
                let sy = (hy - ty).signum();

                if dx.max(dy) == 2 {
                    if hx == tx || hy == ty {
                        knots[i + 1].0 = tail.0 + sx as i64;
                        knots[i + 1].1 = tail.1 + sy as i64;
                    } else {
                        knots[i + 1].0 = tail.0 + sx as i64;
                        knots[i + 1].1 = tail.1 + sy as i64;
                    }
                }
            }

            let (tx, ty) = knots[N - 1];
            tail_positions.insert((tx, ty));
        }
    }

    tail_positions.len() as i64
}

fn parse(input: &str) -> Vec<(i8, i8, u8)> {
    input
        .lines()
        .map(|l| {
            let (dir, amount_str) = l.split_once(" ").unwrap();
            let amount = amount_str.parse().unwrap();
            match dir {
                "U" => (0, 1, amount),
                "D" => (0, -1, amount),
                "L" => (-1, 0, amount),
                "R" => (1, 0, amount),
                _ => panic!("Unknown dir {dir}"),
            }
        })
        .collect()
}
