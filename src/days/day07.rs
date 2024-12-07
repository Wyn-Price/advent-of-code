use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let eqs = parse(input);
    eqs.into_iter()
        .filter(|eq| {
            let tries = 2_usize.pow((eq.nums.len() - 1) as u32);
            for t in 0..tries {
                let mut n = eq.nums[0];
                for idx in 0..eq.nums.len() - 1 {
                    match (t >> idx) & 1 {
                        0 => n += eq.nums[idx + 1],
                        1 => n *= eq.nums[idx + 1],
                        _ => panic!(""),
                    }
                }
                if n == eq.testval {
                    return true;
                }
            }
            return false;
        })
        .map(|eq| eq.testval)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let eqs = parse(input);
    eqs.into_iter()
        .filter(|eq| {
            let tries = 3_usize.pow((eq.nums.len() - 1) as u32);
            for t in 0..tries {
                let mut n = eq.nums[0];
                for idx in 0..eq.nums.len() - 1 {
                    match (t / 3_usize.pow(idx as u32)) % 3 {
                        0 => n += eq.nums[idx + 1],
                        1 => n *= eq.nums[idx + 1],
                        2 => {
                            // n = (n.to_string() + &eq.nums[idx + 1].to_string())
                            //     .parse()
                            //     .unwrap()
                            let num = eq.nums[idx + 1];
                            let num_digits = (num as f64).log(10.0).floor() as u32 + 1;
                            n = n * 10_i64.pow(num_digits) + num;
                        }
                        _ => panic!(""),
                    }
                }
                if n == eq.testval {
                    return true;
                }
            }
            return false;
        })
        .map(|eq| eq.testval)
        .sum()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Equations {
    testval: i64,
    nums: Vec<i64>,
}

fn parse(input: &str) -> Vec<Equations> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let (sop, snums) = l.split_once(": ").unwrap();
            return Equations {
                testval: sop.parse().unwrap(),
                nums: snums
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            };
        })
        .collect_vec()
}
