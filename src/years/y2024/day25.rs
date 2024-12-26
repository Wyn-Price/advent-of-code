use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (locks, keys) = parse(input);
    dbg!(&locks, &keys);
    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(l, k)| {
            for i in 0..l.len() {
                if l[i] + k[i] > 5 {
                    return false;
                }
            }
            return true;
        })
        .count() as i64
}

fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut locks = vec![];
    let mut keys = vec![];
    input.split("\n\n").for_each(|g| {
        let grid_str = g.lines().collect_vec();
        let is_lock = grid_str[0] == "#####";
        let is_key = grid_str[grid_str.len() - 1] == "#####";
        if is_key == is_lock {
            panic!("Invalid: {is_key} {is_lock}");
        }

        let grid = g.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let mut heights = vec![];
        for x in 0..grid[0].len() {
            heights.push((0..grid.len()).filter(|&y| grid[y][x] == '#').count() as i32 - 1);
        }

        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    });

    return (locks, keys);
}
