use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let set = parse(input);

    let mut sum = 0;

    let copy = set.iter().copied().collect::<Vec<_>>();
    for (x, y) in copy {
        let mut around = 0;
        for xi in -1..=1 {
            for yi in -1..=1 {
                if xi == 0 && yi == 0 {
                    continue;
                }

                if set.contains(&(x + xi, y + yi)) {
                    around += 1;
                }
            }
        }

        if around < 4 {
            sum += 1;
        }
    }

    return sum;
}

pub fn part_b(input: &str) -> i64 {
    let mut set = parse(input);

    let mut sum = 0;

    loop {
        let mut changed = false;
        let copy = set.iter().copied().collect::<Vec<_>>();
        for (x, y) in copy {
            if !set.contains(&(x, y)) {
                continue;
            }
            let mut around = 0;
            for xi in -1..=1 {
                for yi in -1..=1 {
                    if xi == 0 && yi == 0 {
                        continue;
                    }

                    if set.contains(&(x + xi, y + yi)) {
                        around += 1;
                    }
                }
            }

            if around < 4 {
                sum += 1;
                set.remove(&(x, y));
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    return sum;
}

fn parse(input: &str) -> HashSet<(i64, i64)> {
    return input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| s.char_indices().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| *c == '@')
        .map(|(x, y, _)| (x as i64, y as i64))
        .collect();
}
