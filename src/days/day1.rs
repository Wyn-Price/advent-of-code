pub fn part_a(input: &str) -> i64 {
    count_increases(numbers(input))
}

pub fn part_b(input: &str) -> i64 {
    let numbers = numbers(input);
    let windows = (0..numbers.len() - 2)
        .map(|i| numbers[i..i + 3].iter().sum())
        .collect::<Vec<_>>();

    count_increases(windows)
}

fn numbers(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
}

fn count_increases(nums: Vec<i64>) -> i64 {
    let mut count = 0;
    for i in 1..nums.len() {
        if nums[i - 1] < nums[i] {
            count += 1;
        }
    }
    count
}
