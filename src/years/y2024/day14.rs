use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let mut robots = parse(input);
    let w = 101;
    let h = 103;
    for _ in 0..100 {
        for robot in &mut robots {
            robot.pos.0 += robot.vel.0;
            if robot.pos.0 >= w {
                robot.pos.0 -= w;
            }
            if robot.pos.0 < 0 {
                robot.pos.0 += w;
            }

            robot.pos.1 += robot.vel.1;
            if robot.pos.1 >= h {
                robot.pos.1 -= h;
            }
            if robot.pos.1 < 0 {
                robot.pos.1 += h;
            }
        }
    }

    let wm = (w - 1) / 2;
    let hm = (h - 1) / 2;

    let mut quadrents = vec![vec![], vec![], vec![], vec![]];
    for robot in robots {
        let (px, py) = robot.pos;
        if px == wm || py == hm {
            continue;
        }
        let xquad = if px < wm { 0 } else { 1 };
        let yquad = if py < hm { 0 } else { 2 };
        quadrents[xquad + yquad].push(robot);
    }

    quadrents.into_iter().map(|q| q.len() as i64).product()
}

pub fn part_b(input: &str) -> i64 {
    let mut robots = parse(input);
    let w = 101;
    let h = 103;
    for i in 1..100000 {
        for robot in &mut robots {
            robot.pos.0 += robot.vel.0;
            if robot.pos.0 >= w {
                robot.pos.0 -= w;
            }
            if robot.pos.0 < 0 {
                robot.pos.0 += w;
            }

            robot.pos.1 += robot.vel.1;
            if robot.pos.1 >= h {
                robot.pos.1 -= h;
            }
            if robot.pos.1 < 0 {
                robot.pos.1 += h;
            }
        }

        let positions: HashSet<_> = robots.iter().map(|r| r.pos).collect();

        let check_size = 3;

        // If there is a block of 3x3 robots then it's prob inside a christmas tree
        for y in 0..h - check_size {
            for x in 0..w - check_size {
                if (y..y + check_size)
                    .cartesian_product(x..x + check_size)
                    .all(|k| positions.contains(&k))
                {
                    return i;
                }
            }
        }
    }

    panic!("Not found")
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| Robot {
            pos: (
                c.get(1).unwrap().as_str().parse().unwrap(),
                c.get(2).unwrap().as_str().parse().unwrap(),
            ),
            vel: (
                c.get(3).unwrap().as_str().parse().unwrap(),
                c.get(4).unwrap().as_str().parse().unwrap(),
            ),
        })
        .collect()
}
