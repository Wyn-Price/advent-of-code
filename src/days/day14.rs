use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let map = parse(input);
    compute(map, true)
}

pub fn part_b(input: &str) -> i64 {
    let map = parse(input);
    compute(map, false)
}

fn compute(mut map: HashMap<(usize, usize), State>, part_a: bool) -> i64 {
    let mut simulations = 0;
    let lowest = map.keys().map(|&(_, y)| y).max().unwrap();

    loop {
        let mut new_sand = (500_usize, 0_usize);

        let settled = loop {
            if new_sand.1 > lowest {
                break true;
            }

            let new_pos = vec![0, -1, 1]
                .into_iter()
                .map(|d| ((new_sand.0 as i32 + d) as usize, new_sand.1 + 1))
                .find(|p| !map.contains_key(p));

            match new_pos {
                Some(pos) => new_sand = pos,
                None => break false,
            }
        };

        // Whatever
        if (part_a && settled) || (!part_a && map.contains_key(&(500, 0))) {
            break;
        }
        map.insert(new_sand, State::Sand);
        simulations += 1;
    }
    simulations
}

fn parse(input: &str) -> HashMap<(usize, usize), State> {
    input
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(",").unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .collect::<Vec<_>>()
                .as_slice()
                .array_windows::<2>()
                .flat_map(|&[l, r]| {
                    let mut vec: Vec<(usize, usize)> = vec![];
                    for x in l.0.min(r.0)..=l.0.max(r.0) {
                        for y in l.1.min(r.1)..=l.1.max(r.1) {
                            vec.push((x, y));
                        }
                    }
                    vec
                })
                .collect::<Vec<_>>()
        })
        .map(|w| (w, State::Rock))
        .collect()
}

#[derive(Debug)]
enum State {
    Rock,
    Sand,
}
