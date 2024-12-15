use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    let heights = parse(input);

    let w = heights[0].len();
    let h = heights.len();

    let mut total: HashSet<(usize, usize)> = HashSet::new();

    add_visible_pos(&heights, &mut total, w, h, false, false);
    add_visible_pos(&heights, &mut total, w, h, true, false);
    add_visible_pos(&heights, &mut total, h, w, false, true);
    add_visible_pos(&heights, &mut total, h, w, true, true);

    (total.len() + w + w + h + h - 4) as i64
}

fn add_visible_pos(
    heights: &Vec<Vec<u8>>,
    totals: &mut HashSet<(usize, usize)>,
    a_size: usize,
    b_size: usize,
    flip: bool,
    invert: bool,
) {
    for a in 1..a_size - 1 {
        let mut max = 0;
        for bb in 0..b_size {
            let b = if flip { b_size - bb - 1 } else { bb };

            let (va, vb) = if invert { (b, a) } else { (a, b) };

            let height = heights[vb][va];
            if height > max {
                max = height;
                if b != 0 && b != b_size - 1 {
                    totals.insert((va, vb));
                }
            }
        }
    }
}

pub fn part_b(input: &str) -> i64 {
    let heights = parse(input);

    heights
        .iter()
        .enumerate()
        .map(|(y, ln)| {
            ln.iter()
                .copied()
                .enumerate()
                .map(|(px, _)| {
                    let py = *(&y); // We don't want this closure to take ownership of y. There is a better way to do this.
                    let l = get_direction_score(&heights, (px, py), (0, -1));
                    let r = get_direction_score(&heights, (px, py), (0, 1));
                    let u = get_direction_score(&heights, (px, py), (-1, 0));
                    let d = get_direction_score(&heights, (px, py), (1, 0));
                    l * r * u * d
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap() as i64
}

fn get_direction_score(heights: &Vec<Vec<u8>>, start: (usize, usize), dir: (i8, i8)) -> i32 {
    let height = heights[start.0][start.1];
    let mut cursor = (
        (start.0 as i8 + dir.0) as usize,
        (start.1 as i8 + dir.1) as usize,
    );
    let mut counter = 0;
    let w = heights[0].len();
    let h = heights.len();

    while cursor.0 < w && cursor.1 < h {
        counter += 1;
        if heights[cursor.0 as usize][cursor.1 as usize] >= height {
            break;
        }
        cursor = (
            (cursor.0 as i8 + dir.0) as usize,
            (cursor.1 as i8 + dir.1) as usize,
        );
    }

    counter
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}
