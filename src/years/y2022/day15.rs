use scan_fmt::scan_fmt;

use std::collections::HashSet;

// Lol who needs a struct
// min xy, max xy -> x, y, diff
fn compute_areas(
    sensors: &Vec<(i32, i32, i32, i32)>,
) -> Vec<((i32, i32, i32, i32), (i32, i32, i32))> {
    let mut areas: Vec<((i32, i32, i32, i32), (i32, i32, i32))> = vec![];
    for &(sx, sy, bx, by) in sensors {
        let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as i32;
        areas.push(((sx - dist, sy - dist, sx + dist, sy + dist), (sx, sy, dist)));
    }
    areas
}

pub fn part_a(input: &str) -> i64 {
    let (sensors, beacons) = parse(input);
    let areas = compute_areas(&sensors);

    let min_x = sensors.iter().map(|&s| s.0).min().unwrap();
    let max_x = sensors.iter().map(|&s| s.1).max().unwrap();
    let max_diff = areas.iter().map(|&(_, (_, _, d))| d).max().unwrap();

    let mut count = 0;

    let y_pos: i32 = 2000000;
    for d in min_x - max_diff..=max_x + max_diff {
        let pos = &(d, y_pos);
        for (_, (sx, sy, diff)) in &areas {
            if sx.abs_diff(pos.0) as i32 + sy.abs_diff(pos.1) as i32 <= *diff {
                if !beacons.contains(pos) {
                    count += 1;
                }
                break;
            }
        }
    }

    count.into()
}

pub fn part_b(input: &str) -> i64 {
    let (sensors, _) = parse(input);
    let areas = compute_areas(&sensors);

    let y_pos: i32 = 4000000;
    for y in 0..=y_pos {
        let mut ranges = vec![];

        for (_, (sx, sy, diff)) in &areas {
            let diff_at_y = diff - (sy - y).abs();

            let lx = (sx - diff_at_y).clamp(0, y_pos);
            let gx = (sx + diff_at_y).clamp(0, y_pos);

            ranges.push((lx, gx));
        }

        ranges.sort_by_key(|&(l, _)| l);

        let mut latest: i64 = 0;
        for range in ranges {
            if range.0 as i64 > latest {
                return ((latest + 1) * 4000000 + y as i64).into();
            }

            latest = latest.max(range.1 as i64);
        }

        if y % 100000 == 0 {
            println!("{y}");
        }
    }

    panic!("Could not find point")
}

fn parse(input: &str) -> (Vec<(i32, i32, i32, i32)>, Vec<(i32, i32)>) {
    let mut sensors: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();

    input.lines().for_each(|l| {
        let (sx, sy, bx, by) = scan_fmt!(
            l,
            "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();
        sensors.insert((sx, sy, bx, by));
        beacons.insert((bx, by));
    });

    return (sensors.into_iter().collect(), beacons.into_iter().collect());
}
