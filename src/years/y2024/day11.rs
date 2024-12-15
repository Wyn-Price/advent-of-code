use std::collections::HashMap;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let mut nums = parse(input);
    for _ in 0..25 {
        let mut new = vec![];
        for idx in 0..nums.len() {
            if nums[idx] == 0 {
                new.push(1);
            } else {
                let num_digits = (nums[idx] as f64).log(10.0).floor() as u32 + 1;
                if num_digits % 2 == 0 {
                    let half = num_digits / 2;
                    let left = nums[idx] / 10_i64.pow(half);
                    let right = nums[idx] % 10_i64.pow(half);

                    new.push(left);
                    new.push(right);
                } else {
                    new.push(nums[idx] * 2024);
                }
            }
        }
        nums = new;
    }

    nums.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let data = parse(input);
    let mut nums_freq: HashMap<i64, i64> = data.into_iter().map(|n| (n, 1)).collect();

    for _ in 0..75 {
        let mut new = HashMap::new();

        let keys = nums_freq.keys().collect_vec();
        for idx in 0..nums_freq.keys().len() {
            let val = *keys[idx];
            let count = nums_freq[&val];
            let mut modify_fn = |v| {
                new.entry(v).and_modify(|e| *e += count).or_insert(count);
            };

            if val == 0 {
                modify_fn(1);
            } else {
                let num_digits = (val as f64).log(10.0).floor() as u32 + 1;
                if num_digits % 2 == 0 {
                    let half = num_digits / 2;
                    let left = val / 10_i64.pow(half);
                    let right = val % 10_i64.pow(half);

                    modify_fn(left);
                    modify_fn(right);
                } else {
                    modify_fn(val * 2024);
                }
            }
        }

        nums_freq = new;
    }

    nums_freq.into_values().sum()
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .flat_map(|l| l.split_whitespace().map(|p| p.parse().unwrap()))
        .collect()
}
