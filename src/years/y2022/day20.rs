pub fn part_a(input: &str) -> i64 {
    compute(parse(input), 1, 1)
}

pub fn part_b(input: &str) -> i64 {
    compute(parse(input), 811589153, 10)
}

fn compute(nums: Vec<i64>, key: i64, repeat: i64) -> i64 {
    let input: Vec<_> = nums.into_iter().map(|i| i * key).enumerate().collect();
    let mut output = input.clone();
    for _ in 0..repeat {
        for i in 0..input.len() {
            let (start_idx, val) = input[i];
            let current_pos = output.iter().position(|&(idx, _)| idx == i).unwrap();

            let len = output.len();
            let new_pos = (current_pos as i64 + val).rem_euclid(len as i64 - 1);

            output.remove(current_pos);
            output.insert(new_pos as usize, (start_idx, val));
        }
    }

    let zero_pos = output.iter().position(|&(_, v)| v == 0).unwrap();

    [1000, 2000, 3000]
        .into_iter()
        .map(|i| zero_pos + i)
        .map(|i| output[i % output.len()].1)
        .sum()
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
