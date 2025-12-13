use std::collections::HashMap;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let nodes = parse(input);

    return count_route("you", "out", &nodes, &mut HashMap::new());
}

fn count_route<'a>(
    node: &'a str,
    target: &'a str,
    nodes: &HashMap<&str, Vec<&'a str>>,
    node_cache: &mut HashMap<&'a str, i64>,
) -> i64 {
    if node == target {
        return 1;
    }
    if node == "out" {
        return 0;
    }

    if let Some(&cached) = node_cache.get(node) {
        return cached;
    }

    let mut sum = 0;
    for &child in &nodes[&node] {
        sum += count_route(child, target, nodes, node_cache);
    }
    node_cache.insert(node, sum);
    return sum;
}

pub fn part_b(input: &str) -> i64 {
    let nodes = parse(input);

    let mut rev_graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for (&node, children) in nodes.iter() {
        for &child in children {
            rev_graph.entry(child).or_default().push(node);
        }
    }

    let svr_dac = count_route("svr", "dac", &nodes, &mut HashMap::new());
    let dac_fft = count_route("dac", "fft", &nodes, &mut HashMap::new());
    let fft_out = count_route("fft", "out", &nodes, &mut HashMap::new());

    let svr_fft = count_route("svr", "fft", &nodes, &mut HashMap::new());
    let fft_dac = count_route("fft", "dac", &nodes, &mut HashMap::new());
    let dac_out = count_route("dac", "out", &nodes, &mut HashMap::new());

    return svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out;
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|l| {
            let (key, vals) = l.split_once(": ").unwrap();
            return (key, vals.split_whitespace().collect_vec());
        })
        .collect()
}
