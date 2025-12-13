use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (presents, regions) = parse(input);
    let mut fit = 0;

    for (w, h, counts) in regions {
        let to_place = counts
            .iter()
            .enumerate()
            .flat_map(|(i, &c)| std::iter::repeat(i).take(c))
            .collect::<Vec<_>>();

        let total_present_cells: usize = to_place
            .iter()
            .map(|&i| {
                presents[i]
                    .iter()
                    .map(|row| row.iter().filter(|&&c| c).count())
                    .sum::<usize>()
            })
            .sum();

        if total_present_cells > w * h {
            continue;
        }

        let mut grid = vec![vec![false; w]; h];
        if dfs(&mut grid, w, h, &presents, &to_place, 0) {
            fit += 1;
        }
    }

    fit
}

fn dfs(
    grid: &mut Vec<Vec<bool>>,
    width: usize,
    height: usize,
    presents: &Vec<Present>,
    to_place: &Vec<usize>,
    idx: usize,
) -> bool {
    if idx == to_place.len() {
        return true;
    }

    let present: &Present = &presents[to_place[idx]];

    (0..4).cartesian_product(0..2).any(|(rotation, flip)| {
        let mut shape = rotate(present, rotation);
        if flip == 1 {
            shape = flip_horizontal(&shape);
        }

        let sh = shape.len();
        let sw = shape[0].len();
        if sh > height || sw > width {
            return false;
        }

        (0..=height - sh)
            .cartesian_product(0..=width - sw)
            .any(|(y, x)| {
                if (0..sh)
                    .cartesian_product(0..sw)
                    .all(|(dy, dx)| !shape[dy][dx] || !grid[y + dy][x + dx])
                {
                    (0..sh)
                        .cartesian_product(0..sw)
                        .filter(|&(dy, dx)| shape[dy][dx])
                        .for_each(|(dy, dx)| grid[y + dy][x + dx] = true);

                    let res = dfs(grid, width, height, presents, to_place, idx + 1);

                    (0..sh)
                        .cartesian_product(0..sw)
                        .filter(|&(dy, dx)| shape[dy][dx])
                        .for_each(|(dy, dx)| grid[y + dy][x + dx] = false);

                    res
                } else {
                    false
                }
            })
    })
}

fn rotate(shape: &Present, times: usize) -> Present {
    let mut result = shape.to_vec();
    for _ in 0..times {
        result = (0..result[0].len())
            .map(|x| (0..result.len()).rev().map(|y| result[y][x]).collect())
            .collect();
    }
    result
}

fn flip_horizontal(shape: &Present) -> Present {
    shape
        .iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

pub fn part_b(_input: &str) -> i64 {
    panic!("Part B not implemented yet");
}

type Present = Vec<Vec<bool>>;
type Region = (usize, usize, Vec<usize>);

fn parse(input: &str) -> (Vec<Present>, Vec<Region>) {
    let mut sections = input.split("\n\n").collect::<Vec<_>>();
    let regions_section = sections.pop().unwrap();

    let presents = sections
        .iter()
        .map(|section| {
            let mut lines = section.lines();
            let _ = lines.next(); // skip index line
            lines
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect::<Present>()
        })
        .collect::<Vec<_>>();

    let regions = regions_section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (size, counts) = line.split_once(':').unwrap();
            let (ws, hs) = size.split_once('x').unwrap();
            let w = ws.parse().unwrap();
            let h = hs.parse().unwrap();
            let counts = counts
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            (w, h, counts)
        })
        .collect::<Vec<_>>();

    (presents, regions)
}
