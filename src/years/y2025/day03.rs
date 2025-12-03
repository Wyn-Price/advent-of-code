pub fn part_a(input: &str) -> i64 {
    let batteries = parse(input);
    let mut sum = 0;

    for bat in batteries {
        let mut max_index = None;
        for i in 0..bat.len() - 1 {
            if max_index.is_none_or(|max| bat[i] > bat[max]) {
                max_index = Some(i);
            }
        }

        let mut max_index2 = None;
        for i in max_index.unwrap() + 1..bat.len() {
            if max_index2.is_none_or(|max| bat[i] > bat[max]) {
                max_index2 = Some(i);
            }
        }

        sum += bat[max_index.unwrap()] as i64 * 10 + bat[max_index2.unwrap()] as i64;
    }

    return sum;
}

pub fn part_b(input: &str) -> i64 {
    let batteries = parse(input);
    let mut sum = 0;

    for bat in batteries {
        let mut bat_sum = 0;
        let mut next_index = 0;
        for s in 0..12 {
            let digits_left = 11 - s;

            let mut max_index = None;
            for i in next_index..bat.len() - digits_left {
                if max_index.is_none_or(|max| bat[i] > bat[max]) {
                    max_index = Some(i);
                }
            }

            next_index = max_index.unwrap() + 1;

            bat_sum = bat_sum * 10 + bat[max_index.unwrap()] as i64;
        }

        sum += bat_sum;
    }

    return sum;
}

fn parse(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}
