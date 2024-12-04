use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let grid = parse(input);
    let w = grid[0].len();
    let h = grid.len();
    let mut counter = 0;

    for x in 0..w {
        for y in 0..h {
            let rules: [[(i32, i32); 4]; 8] = [
                [(0, 0), (0, 1), (0, 2), (0, 3)],
                [(0, 3), (0, 2), (0, 1), (0, 0)],
                [(0, 0), (1, 0), (2, 0), (3, 0)],
                [(3, 0), (2, 0), (1, 0), (0, 0)],
                [(0, 0), (1, 1), (2, 2), (3, 3)],
                [(3, 3), (2, 2), (1, 1), (0, 0)],
                [(0, 0), (1, -1), (2, -2), (3, -3)],
                [(3, -3), (2, -2), (1, -1), (0, 0)],
            ];
            for rule in rules {
                if test_rule(&grid, y as i32, x as i32, rule).unwrap_or(false) {
                    counter += 1;
                }
            }
        }
    }

    counter
}

pub fn part_b(input: &str) -> i64 {
    let grid = parse(input);
    let w = grid[0].len();
    let h = grid.len();
    let mut counter = 0;

    for x in 0..w {
        for y in 0..h {
            if test_xmas(&grid, y as i32, x as i32).unwrap_or(false) {
                counter += 1;
            }
        }
    }

    counter
}

fn test_xmas(grid: &Vec<Vec<char>>, y: i32, x: i32) -> Option<bool> {
    if *grid.get(y as usize)?.get(x as usize)? != 'A' {
        return None;
    }

    let mut or = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

    let yi = y as i32;
    let xi = x as i32;

    for _ in 0..4 {
        or.rotate_left(1);

        if *grid
            .get((yi + or[0].1) as usize)?
            .get((xi + or[0].0) as usize)?
            == 'M'
            && *grid
                .get((yi + or[1].1) as usize)?
                .get((xi + or[1].0) as usize)?
                == 'M'
            && *grid
                .get((yi + or[2].1) as usize)?
                .get((xi + or[2].0) as usize)?
                == 'S'
            && *grid
                .get((yi + or[3].1) as usize)?
                .get((xi + or[3].0) as usize)?
                == 'S'
        {
            return Some(true);
        }
    }

    None
}

fn test_rule(grid: &Vec<Vec<char>>, y: i32, x: i32, ele: [(i32, i32); 4]) -> Option<bool> {
    Some(
        *grid
            .get((y + ele[0].1) as usize)?
            .get((x + ele[0].0) as usize)?
            == 'X'
            && *grid
                .get((y + ele[1].1) as usize)?
                .get((x + ele[1].0) as usize)?
                == 'M'
            && *grid
                .get((y + ele[2].1) as usize)?
                .get((x + ele[2].0) as usize)?
                == 'A'
            && *grid
                .get((y + ele[3].1) as usize)?
                .get((x + ele[3].0) as usize)?
                == 'S',
    )
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}
