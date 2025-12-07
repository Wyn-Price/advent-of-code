use std::collections::HashSet;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (start, height, pos) = parse(input);

    let mut beams = HashSet::new();
    beams.insert(start.0);

    let mut times_split = 0;

    for y in 1..height {
        let mut new_beams = HashSet::new();
        for x in beams {
            if pos.contains(&(x, y)) {
                new_beams.insert(x - 1);
                new_beams.insert(x + 1);
                times_split += 1
            } else {
                new_beams.insert(x);
            }
        }
        beams = new_beams;
    }

    return times_split;
}

pub fn part_b(input: &str) -> i64 {
    let (start, height, pos) = parse(input);

    let mut beams = vec![(start.0, 1_i64)];

    for y in 1..height {
        println!("{y} / {height}");
        let mut new_beams = vec![];
        for (x, count) in beams {
            if pos.contains(&(x, y)) {
                new_beams.push((x - 1, count));
                new_beams.push((x + 1, count));
            } else {
                new_beams.push((x, count));
            }
        }
        beams = new_beams
            .into_iter()
            .into_group_map_by(|(x, _)| *x)
            .into_iter()
            .map(|(x, values)| (x, values.into_iter().map(|(_, count)| count).sum()))
            .collect_vec();
    }

    return beams.into_iter().map(|b| b.1).sum();
}

fn parse(input: &str) -> ((i64, i64), i64, HashSet<(i64, i64)>) {
    let mut start = None;
    let mut positions = HashSet::new();

    input.lines().enumerate().for_each(|(y, l)| {
        l.char_indices().for_each(|(x, c)| {
            if c == 'S' {
                start = Some((x as i64, y as i64));
            } else if c == '^' {
                positions.insert((x as i64, y as i64));
            }
        })
    });

    return (start.unwrap(), input.lines().count() as i64, positions);
}
