use itertools::Itertools;

type Area = Vec<Vec<bool>>;

pub fn part_a(input: &str) -> i64 {
    let areas = parse(input);
    areas.into_iter().map(|area| rank(area, 0)).sum::<i64>()
}

pub fn part_b(input: &str) -> i64 {
    let areas = parse(input);
    areas.into_iter().map(|area| rank(area, 1)).sum::<i64>()
}

fn rank_dimension(
    area: &Area,
    row_differ: usize,
    max: usize,
    count_differences: impl Fn(&Area, usize, usize) -> usize,
) -> i64 {
    // List of left/above side positions of the mirror
    // Add tests between the first two and last two elements, as that will be missed when fixing
    let mut test = vec![0, max - 2];
    for i in 0..max {
        for i2 in i + 1..max {
            if count_differences(&area, i, i2) == 0 {
                test.push((i + i2 - 1) / 2)
            }
        }
    }

    test.into_iter()
        .unique()
        .filter(|t| {
            let total_not_hit = (0..t + 1)
                .map(|i| {
                    // Return false when equal, true when not equal
                    let i1 = t - i;
                    let i2 = t + i + 1;

                    // If it's out of bounds, we can assume it does reflect
                    if i2 >= max {
                        return 0;
                    }

                    return count_differences(&area, i1, i2);
                })
                .sum::<usize>();

            return total_not_hit == row_differ;
        })
        .map(|t| 1 + t as i64)
        .sum::<i64>()
}

fn rank(area: Area, row_differ: usize) -> i64 {
    let height = area.len();
    let width = area[0].len();

    rank_dimension(&area, row_differ, width, col_ne_c)
        + 100 * rank_dimension(&area, row_differ, height, row_ne_c)
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
