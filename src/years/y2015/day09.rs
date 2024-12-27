use std::collections::HashMap;

use itertools::Itertools;

pub fn part_a(input: &str) -> i32 {
    let map = parse(input);
    map.keys()
        .permutations(map.keys().len())
        .filter_map(|keys| {
            let mut dist = 0;
            for (&f, &t) in keys.into_iter().tuple_windows() {
                dist += map.get(f)?.get(t)?;
            }
            Some(dist)
        })
        .min()
        .unwrap()
}

pub fn part_b(input: &str) -> i32 {
    let map = parse(input);
    map.keys()
        .permutations(map.keys().len())
        .filter_map(|keys| {
            let mut dist = 0;
            for (&f, &t) in keys.into_iter().tuple_windows() {
                dist += map.get(f)?.get(t)?;
            }
            Some(dist)
        })
        .min()
        .unwrap()
}

fn parse(input: &str) -> HashMap<&str, HashMap<&str, i32>> {
    input
        .lines()
        .flat_map(|l| {
            let (f, _, t, _, d) = l.split_whitespace().collect_tuple().unwrap();
            let dv = d.parse::<i32>().unwrap();
            return vec![(f, t, dv), (t, f, dv)];
        })
        .fold(HashMap::new(), |mut map, (f, t, d)| {
            map.entry(f).or_insert_with(HashMap::new).insert(t, d);
            map
        })
}
