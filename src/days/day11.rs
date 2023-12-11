use itertools::Itertools;

type Input = Vec<(i64, i64)>;

pub fn part_a(input: &str) -> i64 {
    let mut input = parse(input);
    expand_space(&mut input, 1);
    compute_spaces(input)
}

pub fn part_b(input: &str) -> i64 {
    let mut input = parse(input);
    expand_space(&mut input, 999_999);
    compute_spaces(input)
}

fn compute_spaces(input: Input) -> i64 {
    input
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum::<u64>() as i64
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|&(_, c)| c == '#')
                .map(|(x, _)| (x as i64, y as i64))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn expand_space(input: &mut Input, size: i64) {
    let width = *input.iter().map(|(x, _)| x).max().unwrap();
    let height = *input.iter().map(|(_, y)| y).max().unwrap();

    let mut x_spaces = (0..width)
        .filter(|&tx| !input.iter().any(|&(x, _)| tx == x))
        .collect::<Vec<_>>();

    x_spaces.sort();

    for (i, &xs) in x_spaces.iter().enumerate() {
        input
            .iter_mut()
            .filter(|(x, _)| xs < *x - size * i as i64)
            .for_each(|(x, _)| *x += size)
    }

    let mut y_spaces = (0..height)
        .filter(|&ty| !input.iter().any(|&(_, y)| ty == y))
        .collect::<Vec<_>>();

    y_spaces.sort();

    for (i, &ys) in y_spaces.iter().enumerate() {
        input
            .iter_mut()
            .filter(|(_, y)| ys < *y - size * i as i64)
            .for_each(|(_, y)| *y += size)
    }
}
