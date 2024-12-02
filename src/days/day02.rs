use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let grid = parse(input);
    grid.into_iter()
        .filter(|g| {
            let increasing = g[0] < g[1];
            g.into_iter()
                .tuple_windows()
                .all(|(c, n)| c.abs_diff(*n) <= 3 && (if increasing { c < n } else { n < c }))
        })
        .count() as i64
}

pub fn part_b(input: &str) -> i64 {
    let grid = parse(input);
    grid.into_iter()
        .filter(|g| {
            (0..=g.len())
                .map(|i| {
                    let mut new_vec = g.clone();
                    if i != g.len() {
                        new_vec.remove(i);
                    }
                    new_vec
                })
                .any(|l| {
                    let increasing = l[0] < l[1];
                    let r = l.into_iter().tuple_windows().all(|(c, n)| {
                        c.abs_diff(n) <= 3 && (if increasing { c < n } else { n < c })
                    });
                    return r;
                })
        })
        .count() as i64
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}
