use itertools::Itertools;

type Area = Vec<Vec<bool>>;

pub fn part_a(input: &str) -> i64 {
    let areas = parse(input);
    areas
        .into_iter()
        .enumerate()
        .map(|(b, area)| {
            println!("{b}");
            rank(area, false)
        })
        .sum::<i64>()
}

pub fn part_b(input: &str) -> i64 {
    let areas = parse(input);
    areas
        .into_iter()
        .enumerate()
        .map(|(b, area)| {
            println!("{b}");
            rank(area, true)
        })
        .sum::<i64>()
}

fn rank(area: Area, fix: bool) -> i64 {
    let height = area.len();
    let width = area[0].len();

    let mut sum = 0;

    // Check vertical mirrors
    // List of left side positions of the mirror
    // Add tests between the first two and last two elements, as that will be missed when fixing
    let mut test_vertical = vec![0, width - 2];
    for x in 0..width {
        for xc in x + 1..width {
            if col_eq(&area, x, xc) {
                test_vertical.push((x + xc - 1) / 2)
            }
        }
    }

    sum += test_vertical
        .into_iter()
        .unique()
        .filter_map(|tv| {
            let not_hit = (0..tv + 1)
                .map(|y| {
                    // Return false when equal, true when not equal
                    let x1 = tv - y;
                    let x2 = tv + y + 1;

                    if x2 >= width {
                        return 0;
                    }

                    return col_ne_c(&area, x1, x2);
                })
                .collect_vec();
            if fix {
                if not_hit.into_iter().sum::<usize>() == 1 {
                    println!("  Flip V {tv}");
                    return Some(1 + tv as i64);
                }
            } else if !not_hit.into_iter().any(|d| d != 0) {
                println!("  Vertical {tv}");
                return Some(1 + tv as i64);
            }
            return None;
        })
        .sum::<i64>();

    // Check horizontal mirrors
    // List of upwards side positions of the mirror
    // Add tests between the first two and last two elements, as that will be missed when fixing
    let mut test_horizontal = vec![0, height - 2];
    for y in 0..height {
        for yc in y + 1..height {
            if row_eq(&area, y, yc) {
                test_horizontal.push((y + yc - 1) / 2)
            }
        }
    }

    sum += test_horizontal
        .into_iter()
        .unique()
        .filter_map(|th| {
            let not_hit = (0..th + 1)
                .map(|x| {
                    // Return false when equal, true when not equal
                    let y1 = th - x;
                    let y2 = th + x + 1;

                    if y2 >= height {
                        return 0;
                    }

                    return row_ne_c(&area, y1, y2);
                })
                .collect_vec();
            if fix {
                if not_hit.into_iter().sum::<usize>() == 1 {
                    println!("  Flip H {th}");
                    return Some(1 + th as i64);
                }
            } else if !not_hit.into_iter().any(|d| d != 0) {
                println!("  Horizontal {th}");
                return Some(1 + th as i64);
            }
            return None;
        })
        .sum::<i64>()
        * 100;

    sum
}

fn col_eq(area: &Area, x1: usize, x2: usize) -> bool {
    !(0..area.len()).any(|y| area[y][x1] != area[y][x2])
}

fn row_eq(area: &Area, y1: usize, y2: usize) -> bool {
    !(0..area[0].len()).any(|x| area[y1][x] != area[y2][x])
}

fn col_ne_c(area: &Area, x1: usize, x2: usize) -> usize {
    (0..area.len())
        .filter(|&y| area[y][x1] != area[y][x2])
        .count()
}

fn row_ne_c(area: &Area, y1: usize, y2: usize) -> usize {
    (0..area[0].len())
        .filter(|&x| area[y1][x] != area[y2][x])
        .count()
}

fn parse(input: &str) -> Vec<Area> {
    input
        .split("\n\n")
        .map(|area| {
            area.lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect()
}
