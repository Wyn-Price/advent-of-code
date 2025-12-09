use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let pos = parse(input);

    let map = pos
        .iter()
        .combinations(2)
        .map(|comb| return ((*comb[0], *comb[1]), dist(comb[0], comb[1])))
        .collect::<HashMap<_, _>>();

    let mut circuit: Vec<HashSet<Pos>> = vec![];

    map.keys()
        .sorted_by_cached_key(|&k| map.get(k))
        .take(1000)
        .for_each(|&(a, b)| {
            let mut hits = vec![];

            for (i, cir) in circuit.iter().enumerate() {
                if cir.contains(&a) || cir.contains(&b) {
                    hits.push(i);
                }
            }

            match hits.len() {
                0 => {
                    // new circuit
                    let mut set = HashSet::new();
                    set.insert(a);
                    set.insert(b);
                    circuit.push(set);
                }
                1 => {
                    // extend existing
                    let cir = &mut circuit[hits[0]];
                    cir.insert(a);
                    cir.insert(b);
                }
                _ => {
                    // merge all hit circuits
                    let mut new_set = HashSet::new();
                    for i in hits.into_iter().rev() {
                        let old = circuit.remove(i);
                        new_set.extend(old);
                    }
                    new_set.insert(a);
                    new_set.insert(b);
                    circuit.push(new_set);
                }
            }
        });

    println!("{}", circuit.len());
    println!("{:?}", circuit.iter().map(|s| s.len()).collect_vec());
    circuit
        .into_iter()
        .map(|c| c.len() as i64)
        .sorted()
        .rev()
        .take(3)
        .reduce(|acc, e| acc * e)
        .unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let pos = parse(input);

    let map = pos
        .iter()
        .combinations(2)
        .map(|comb| return ((*comb[0], *comb[1]), dist(comb[0], comb[1])))
        .collect::<HashMap<_, _>>();

    let mut circuit: Vec<HashSet<Pos>> = vec![];

    for &(a, b) in map
        .keys()
        .sorted_by_cached_key(|&k| map.get(k))
        .collect_vec()
    {
        let mut hits = vec![];

        for (i, cir) in circuit.iter().enumerate() {
            if cir.contains(&a) || cir.contains(&b) {
                hits.push(i);
            }
        }

        match hits.len() {
            0 => {
                // new circuit
                let mut set = HashSet::new();
                set.insert(a);
                set.insert(b);
                circuit.push(set);
            }
            1 => {
                // extend existing
                let cir = &mut circuit[hits[0]];
                cir.insert(a);
                cir.insert(b);
            }
            _ => {
                // merge all hit circuits
                let mut new_set = HashSet::new();
                for i in hits.into_iter().rev() {
                    let old = circuit.remove(i);
                    new_set.extend(old);
                }
                new_set.insert(a);
                new_set.insert(b);
                circuit.push(new_set);
            }
        }

        if circuit.len() == 1 && circuit[0].len() == pos.len() {
            return a.0 * b.0;
        }
    }

    panic!("");
}

type Pos = (i64, i64, i64);

fn dist(pos1: &Pos, pos2: &Pos) -> i64 {
    (pos1.0 - pos2.0).pow(2) + (pos1.1 - pos2.1).pow(2) + (pos1.2 - pos2.2).pow(2)
}

fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}
