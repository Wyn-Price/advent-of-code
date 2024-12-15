pub fn part_a(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|his| {
            let mut diffs = compute_differences(his);

            // Insert the 0
            diffs.last_mut().unwrap().push(0);

            for i in 1..diffs.len() {
                let r = diffs.len() - i - 1;
                let lnl = diffs[r].len();
                let new = diffs[r][lnl - 1] + diffs[r + 1][lnl - 1];
                diffs[r].push(new);
            }

            diffs[0].last().unwrap().clone()
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|his| {
            let mut diffs = compute_differences(his);

            // Insert the 0
            diffs.last_mut().unwrap().insert(0, 0);

            for i in 1..diffs.len() {
                let r = diffs.len() - i - 1;
                let new = diffs[r][0] - diffs[r + 1][0];
                diffs[r].insert(0, new);
            }

            diffs[0].first().unwrap().clone()
        })
        .sum()
}

fn compute_differences(values: Vec<i64>) -> Vec<Vec<i64>> {
    let mut nums = values;
    let mut diffs = vec![];

    while nums.iter().any(|&n| n != 0) {
        diffs.push(nums.clone());

        let vals = nums.iter();
        let next = nums.iter().skip(1);
        nums = vals.zip(next).map(|(&cur, &next)| next - cur).collect()
    }

    diffs.push(nums);

    diffs
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|t| t.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}
