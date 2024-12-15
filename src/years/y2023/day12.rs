use std::collections::{HashMap, VecDeque};

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (data, broken) = parse_line(l);
            gather_combs(data)
                .into_iter()
                .filter(|c| naive_comb_matches(c, &broken))
                .count()
        })
        .sum::<usize>() as i64
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            let (mut data, broken) = parse_line(l);

            data.push('?');

            let data_len = data.len();
            let broken_len = broken.len();

            let data5: Vec<_> = data.into_iter().cycle().take(data_len * 5 - 1).collect();
            let broken5: Vec<_> = broken.into_iter().cycle().take(broken_len * 5).collect();

            // gather_combs(data)
            //     .into_iter()
            //     .tuple_combinations()
            //     .filter(|(c1, c2, c3, c4, c5)| {
            //         naive_comb_matches(&format!("{c1}?{c2}?{c3}?{c4}?{c5}"), &broken5)
            //     })
            //     .count()

            let m =
                compute_all_matches(&mut HashMap::new(), data5.as_slice(), 0, broken5.as_slice())
                    as i64;
            println!("{idx} -> {m}");
            return m;
        })
        .sum()
}
fn parse_line(input: &str) -> (Vec<char>, Vec<i32>) {
    let (data, broken) = input.split_once(" ").unwrap();
    (
        data.chars().collect(),
        broken
            .split(",")
            .map(|d| d.parse::<i32>().unwrap())
            .collect(),
    )
}

fn gather_combs(data: Vec<char>) -> Vec<String> {
    let mut combs = vec![String::from("")];

    for c in data {
        if c != '?' {
            combs = combs.iter().map(|s| format!("{}{}", s, c)).collect();
            continue;
        }
        combs = combs
            .iter()
            .flat_map(|s| vec![format!("{}.", s), format!("{}#", s)])
            .collect();
    }

    combs
}

fn naive_comb_matches(data: &String, broken: &Vec<i32>) -> bool {
    let mut deque: VecDeque<_> = broken.clone().into();

    let mut group_size = 0;
    for (i, char) in data.char_indices() {
        if char == '#' {
            group_size += 1;
        }

        // Group end
        if (char == '.' || i == data.len() - 1) && group_size != 0 {
            if deque.is_empty() || group_size != deque[0] {
                return false;
            }
            deque.pop_front();
            group_size = 0;
        }
    }

    return deque.is_empty();
}

fn compute_all_matches(
    cache: &mut HashMap<(usize, i32, usize), i64>,
    data: &[char],
    group_size: i32,
    broken: &[i32],
) -> i64 {
    if data.is_empty() {
        if group_size == 0 && broken.is_empty() {
            return 1;
        }
        if broken.len() == 1 && broken[0] == group_size {
            return 1;
        }
        return 0;
    }

    if broken.is_empty() && group_size != 0 {
        return 0;
    }

    let key = (data.len(), group_size, broken.len());
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let ways = match data[0] {
        '.' => {
            if group_size == 0 {
                compute_all_matches(cache, &data[1..], 0, broken)
            } else if group_size == broken[0] {
                // Pop off last group
                compute_all_matches(cache, &data[1..], 0, &broken[1..])
            } else {
                // group size != broken[0], invalid
                0
            }
        }
        '#' => compute_all_matches(cache, &data[1..], group_size + 1, broken),
        '?' => {
            let mut n = compute_all_matches(cache, &data[1..], group_size + 1, broken);
            // If there was no group, we can prevtend it was a '.'
            if group_size == 0 {
                n += compute_all_matches(cache, &data[1..], 0, broken);
            }
            // If we've reached the end of a group pop off and pretend it was a '.'
            else if group_size == broken[0] {
                n += compute_all_matches(cache, &data[1..], 0, &broken[1..]);
            }

            n
        }
        _ => unreachable!(),
    };

    cache.insert(key, ways);

    return ways;
}
