use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let nums = parse(input);
    nums.into_iter()
        .map(|initial| {
            let mut num = initial;
            for _ in 0..2000 {
                num = evolve(num);
            }
            num
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let nums = parse(input);
    nums.into_iter()
        .map(|initial| {
            let mut map = HashMap::new();
            let mut num = initial;
            let mut prev_price = num % 10;
            let mut prices = [0, 0, 0, 0];
            for _ in 0..2000 {
                num = evolve(num);

                prices.rotate_left(1);
                let price = num % 10;
                let price_diff = num % 10 - prev_price;
                prices[3] = price_diff;
                prev_price = price;

                if prices[0] != 0 && prices[1] != 0 && prices[2] != 0 && prices[3] != 0 {
                    let k = (prices[0], prices[1], prices[2], prices[3]);
                    if !map.contains_key(&k) {
                        map.insert(k, price);
                    }
                }
            }
            return map;
        })
        .fold(HashMap::new(), |mut acc, map| {
            map.into_iter().for_each(|(k, v)| {
                acc.entry(k).and_modify(|e| *e += v).or_insert(v);
            });
            return acc;
        })
        .into_iter()
        .map(|(_, v)| v)
        .max()
        .unwrap()
}

fn evolve(mut num: i64) -> i64 {
    let mut v = num << 6;
    num ^= v;
    num &= 16777215;

    v = num >> 5;
    num ^= v;
    num &= 16777215;

    v = num << 11;
    num ^= v;
    num &= 16777215;

    return num;
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
