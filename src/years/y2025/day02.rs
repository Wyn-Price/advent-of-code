pub fn part_a(input: &str) -> i64 {
    let ranges = parse(input);
    let mut sum = 0;

    for (from, to) in ranges {
        println!("{from} -> {to}");
        for i in from..=to {
            let str = i.to_string();
            if str.len() % 2 == 1 {
                continue;
            }

            let (l, r) = str.split_at(str.len() / 2);
            if l == r {
                println!("{i}");
                sum += i;
            }
        }
    }

    return sum;
}

pub fn part_b(input: &str) -> i64 {
    let ranges = parse(input);
    let mut sum = 0;

    for (from, to) in ranges {
        println!("{from} -> {to}");
        for i in from..=to {
            let str = i.to_string();
            let len = str.len();

            for p in 1..=len / 2 {
                if len % p != 0 {
                    continue;
                }

                let pat = &str[0..p];
                if pat.repeat(len / p) == str {
                    sum += i;
                    break;
                }
            }
        }
    }

    return sum;
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .split(",")
        .into_iter()
        .map(|s| {
            let (a, b) = s.split_once("-").unwrap();
            return (a.parse().unwrap(), b.parse().unwrap());
        })
        .collect()
}
