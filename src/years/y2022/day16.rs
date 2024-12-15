use regex::Regex;
use std::collections::HashMap;

const RE: &str = r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)";

fn parse(input: &str) -> HashMap<u64, (u64, Vec<u64>, String)> {
    let regex = Regex::new(RE).unwrap();
    let mut map = HashMap::new();
    let mut arr = input
        .split("\n")
        .map(|line| regex.captures(line))
        .filter_map(|caps| caps)
        .map(|caps| {
            (
                caps[1].to_string(),
                caps[2].parse::<u64>().unwrap(),
                caps[3].to_string(),
            )
        })
        .collect::<Vec<_>>();
    arr.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));
    for (i, (valve, rate, valves)) in arr.iter().enumerate() {
        let bit = 1 << i;
        let valves_bits = valves
            .split(", ")
            .map(|v| {
                let j = arr.iter().position(|(valve_, _, _)| valve_ == v).unwrap();
                1 << j
            })
            .collect::<Vec<_>>();
        map.insert(bit, (*rate, valves_bits, valve.to_string()));
    }
    map
}

// Floyd-Warshall this bitch
fn shortest_path(graph: HashMap<u64, Vec<u64>>) -> HashMap<u64, HashMap<u64, u64>> {
    let keys: Vec<_> = graph.keys().copied().collect();
    let mut dist_map: HashMap<u64, HashMap<u64, u64>> = keys
        .iter()
        .map(|k| {
            let mut inner_map = HashMap::new();
            for l in &keys {
                inner_map.insert(*l, u64::MAX);
            }
            (*k, inner_map)
        })
        .collect();
    for u in keys.iter() {
        if let Some(v) = graph.get(u) {
            if let Some(inner_map) = dist_map.get_mut(u) {
                for v in v {
                    inner_map.insert(*v, 1);
                }
            }
        }
    }
    for k in keys.iter() {
        if let Some(inner_map) = dist_map.get_mut(k) {
            inner_map.insert(*k, 0);
        }
    }
    for k in keys.iter() {
        for i in keys.iter() {
            for j in keys.iter() {
                if let (Some(i_inner_map), Some(k_inner_map)) = (dist_map.get(i), dist_map.get(k)) {
                    if i_inner_map[k] != u64::MAX && k_inner_map[j] != u64::MAX {
                        let new_val = i_inner_map[k] + k_inner_map[j];
                        if new_val < i_inner_map[j] {
                            if let Some(i_inner_map) = dist_map.get_mut(i) {
                                i_inner_map.insert(*j, new_val);
                            }
                        }
                    };
                }
            }
        }
    }
    dist_map
}

type Cache = HashMap<(u64, u64, u64, bool), u64>;

fn evaluate(input: &str, time: u64, firstrun: bool) -> u64 {
    let data = parse(input);
    // dbg!(&data);
    let dist_map = shortest_path(data.iter().map(|(k, v)| (*k, v.1.clone())).collect());
    let keys: Vec<u64> = data.keys().cloned().collect();
    let flow: HashMap<u64, u64> = data.iter().map(|(k, v)| (*k, v.0)).collect();

    let mut cache = HashMap::new();
    dfs(
        &keys, &flow, &dist_map, time, &mut cache, 1, time, 0, firstrun,
    )
}

fn dfs(
    keys: &Vec<u64>,
    flow: &HashMap<u64, u64>,
    dist_map: &HashMap<u64, HashMap<u64, u64>>,
    time: u64,
    cache: &Cache,
    valve: u64,
    minutes: u64,
    open: u64,
    firstrun: bool,
) -> u64 {
    let cache_key = (valve, minutes, open, firstrun);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }
    let result = keys
        .iter()
        .filter(|k| {
            !open & *k != 0
                && flow.get(k).unwrap() > &0
                && *dist_map.get(&valve).unwrap().get(k).unwrap() < minutes
        })
        .map(|k| {
            let d = *dist_map.get(&valve).unwrap().get(k).unwrap() + 1;
            let timeleft = minutes - d;
            timeleft * flow.get(k).unwrap()
                + dfs(
                    keys,
                    flow,
                    dist_map,
                    time,
                    cache,
                    *k,
                    timeleft,
                    open | *k,
                    firstrun,
                )
        })
        .fold(
            if firstrun {
                dfs(keys, flow, dist_map, time, cache, 1, time, 0, false)
            } else {
                0
            },
            |max, v| if max > v { max } else { v },
        );

    // Suck my dick rust why the fuck is a cache in a recursive function so hard
    unsafe {
        let pnt = cache as *const Cache;
        let mut_pnt = pnt as *mut Cache;
        (*mut_pnt).insert(cache_key, result);
    }

    result
}

pub fn part_a(input: &str) -> i64 {
    evaluate(input, 30, false) as i64
}

pub fn part_b(input: &str) -> i64 {
    evaluate(input, 26, true) as i64
}
