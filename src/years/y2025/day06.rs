use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (line, ops) = parse(input);

    let mut total = 0;
    for column in 0..ops.len() {
        let mut sum = line[0][column];
        for row in 1..line.len() {
            match ops[column] {
                '+' => sum += line[row][column],
                '*' => sum *= line[row][column],
                _ => panic!("{}", ops[column]),
            }
        }

        total += sum;
    }

    return total;
}

pub fn part_b(input: &str) -> i64 {
    let mut total = 0;

    let lines = input.lines().collect_vec();
    let rows = lines[0].len();

    let mut current_nums: Vec<i64> = vec![];
    let mut current_op = None;

    for x in 0..=rows {
        let mut num = String::new();
        for y in 0..lines.len() - 1 {
            let c = lines[y].chars().nth(x).unwrap_or(' ');
            if c != ' ' {
                num.push(c);
            }
        }

        let c = lines[lines.len() - 1].chars().nth(x).unwrap_or(' ');
        if c != ' ' {
            current_op = Some(c);
        }

        if num == "" {
            let mut sum = current_nums.remove(0);
            for ele in current_nums {
                match current_op.unwrap() {
                    '+' => sum += ele,
                    '*' => sum *= ele,
                    _ => panic!(),
                }
            }

            total += sum;
            current_nums = vec![];
            current_op = None;
        } else {
            current_nums.push(num.parse().unwrap());
        }
    }

    return total;
}

fn parse(input: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let mut lines = input.lines().collect_vec();
    let ops = lines.pop().unwrap();

    return (
        lines
            .into_iter()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
        ops.split_whitespace()
            .map(|o| o.chars().nth(0).unwrap())
            .collect_vec(),
    );
}
