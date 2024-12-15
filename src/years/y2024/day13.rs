use itertools::Itertools;
use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let machines = parse(input);
    machines
        .into_iter()
        .filter_map(|m| inv_matrix(m, 0.0))
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let machines = parse(input);
    machines
        .into_iter()
        .filter_map(|m| inv_matrix(m, 10000000000000.0))
        .sum()
}

fn inv_matrix(m: Machine, addition: f64) -> Option<i64> {
    // A*ax + B*bx = px
    // A*ay + B*by = py
    //
    //   [A, * [ax, bx, = [px,
    //    B]    ay, by]    py]
    //
    //   [A, = [px, * [ax, bx, ^-1
    //    B]    py]    ay, by]
    //
    //
    // Matrix :)
    //    [ax, bx, ^-1
    //    ay, by]
    // =
    //    det * [by, -bx,
    //          -ay, ax]
    // Where
    //   det = ax*by - ay*bx
    //
    // So:
    //   A = (by*px - bx*py) / det
    //   B = (-ay*px + ax*py) / det
    let (ax, ay) = m.button_a;
    let (bx, by) = m.button_b;
    let (mut px, mut py) = m.prize;

    px += addition;
    py += addition;

    let det = 1.0 / (ax * by - ay * bx);

    let a = (by * px - bx * py) * det;
    let b = (-ay * px + ax * py) * det;

    // Floating point math :(
    // Check re-doing the calculation in integers work
    // This assumes ax,bx and px can go back to their integer form nicely,
    //      but considering they should have a fract of 0, it should be fine
    let ai = a.round() as i64;
    let bi = b.round() as i64;
    if ai * (ax as i64) + bi * (bx as i64) != (px as i64)
        || ai * (ay as i64) + bi * (by as i64) != (py as i64)
    {
        return None;
    }

    Some((ai * 3 + bi) as i64)
}

struct Machine {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}

fn parse(input: &str) -> Vec<Machine> {
    let reg = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();
    input
        .split("\n\n")
        .map(|s| {
            let (ca, cb, cp) = reg.captures_iter(s).collect_tuple().unwrap();
            Machine {
                button_a: (
                    ca.get(1).unwrap().as_str().parse().unwrap(),
                    ca.get(2).unwrap().as_str().parse().unwrap(),
                ),
                button_b: (
                    cb.get(1).unwrap().as_str().parse().unwrap(),
                    cb.get(2).unwrap().as_str().parse().unwrap(),
                ),
                prize: (
                    cp.get(1).unwrap().as_str().parse().unwrap(),
                    cp.get(2).unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect_vec()
}
