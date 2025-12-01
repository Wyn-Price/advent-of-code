pub fn part_a(input: &str) -> i64 {
    let nums = parse(input);

    let mut dial = 50;
    let mut times_zero = 0;

    for rot in nums {
        dial += rot;
        dial = dial.rem_euclid(100);

        if dial == 0 {
            times_zero += 1;
        }
    }

    times_zero
}

pub fn part_b(input: &str) -> i64 {
    let nums = parse(input);

    let mut dial: i32 = 50;
    let mut times_zero = 0;

    for rot in nums {
        let step = if rot > 0 { 1 } else { -1 };

        for _ in 0..rot.abs() {
            dial = (dial + step).rem_euclid(100);

            if dial == 0 {
                times_zero += 1;
            }
        }
    }

    times_zero
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            if let Some(num) = l.strip_prefix("L") {
                -num.parse::<i64>().unwrap()
            } else if let Some(num) = l.strip_prefix("R") {
                num.parse::<i64>().unwrap()
            } else {
                panic!("Don't knowto handle {l}")
            }
        })
        .collect()
}
