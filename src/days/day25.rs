pub fn part_a(input: &str) -> i64 {
    let sum = parse(input).into_iter().sum();
    println!("{}", dec_to_snafu(sum, 0, ""));
    0
}

pub fn part_b(_input: &str) -> i64 {
    panic!("Merry christmas");
}

fn parse(input: &str) -> Vec<i64> {
    input.trim().lines().map(|l| snafu_to_dec(l)).collect()
}

fn snafu_to_dec(snafu: &str) -> i64 {
    snafu.chars().rev().enumerate().fold(0, |acc, (index, char)| {
        let amount: i64 = match char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unknown {char}")
        };
        let val = amount * 5_i64.pow(index as u32);
        return acc + val;
    })
}

fn dec_to_snafu(remains: i64, leftover: i64, total: &str) -> String {
    if remains == 0 && leftover == 0 {
        return total.to_owned();
    }
    let mut pos = remains % 5 + leftover;

    let overflow = pos >= 5;
    let mut to_carry = if overflow { 1 } else { 0 };
    if overflow {
        pos -= 5;
    }

    let result = match pos {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => {
            to_carry += 1;
            "="
        },
        4 => {
            to_carry += 1;
            "-"
        }
        _ => panic!("Unknown position {pos}")
    };

    return dec_to_snafu(remains / 5, to_carry, &(total.to_owned() + result))
}
