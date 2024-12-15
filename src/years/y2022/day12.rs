use std::collections::HashMap;

use petgraph::{algo::dijkstra, stable_graph::NodeIndex, Graph};

pub fn part_a(input: &str) -> i64 {
    let (graph, start, end, _) = parse(input);

    let path = dijkstra(&graph, start, Some(end), |_| 1);

    let at_end = path.get(&end).unwrap();

    *at_end as i64
}

pub fn part_b(input: &str) -> i64 {
    let (mut graph, _, end, node_datas) = parse(input);
    graph.reverse();

    let path = dijkstra(&graph, end, None, |_| 1);

    dbg!(&path);

    let min = node_datas
        .iter()
        .filter(|&(_, val)| *val == 1)
        .filter_map(|(idx, _)| path.get(idx))
        .min()
        .unwrap();

    *min as i64
}

fn parse(
    input: &str,
) -> (
    Graph<char, ()>,
    NodeIndex,
    NodeIndex,
    HashMap<NodeIndex, u16>,
) {
    let mut graph = Graph::<char, ()>::new();

    let mut nodes: HashMap<(usize, usize), (u16, _)> = HashMap::new();
    let mut nodes_to_data: HashMap<NodeIndex, u16> = HashMap::new();

    let mut start: Option<_> = None;
    let mut end: Option<_> = None;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, raw_c)| {
            let mut c = raw_c;
            if c == 'S' {
                c = 'a';
            } else if c == 'E' {
                c = 'z';
            }

            let idx = (c as u8 - b'a') as u16 + 1;

            let node = graph.add_node(raw_c);

            if raw_c == 'S' {
                start = Some(node);
            }

            if raw_c == 'E' {
                end = Some(node);
            }

            if x != 0 {
                nodes.get(&(x - 1, y)).map(|&(left, left_node)| {
                    if left == idx - 1 || left >= idx {
                        graph.add_edge(left_node, node, ());
                    }

                    if idx == left - 1 || idx >= left {
                        graph.add_edge(node, left_node, ());
                    }
                });
            }

            if y != 0 {
                nodes.get(&(x, y - 1)).map(|&(top, top_node)| {
                    if top == idx - 1 || top >= idx {
                        graph.add_edge(top_node, node, ());
                    }

                    if idx == top - 1 || idx >= top {
                        graph.add_edge(node, top_node, ());
                    }
                });
            }
            nodes.insert((x, y), (idx, node));
            nodes_to_data.insert(node, idx);
        })
    });

    (graph, start.unwrap(), end.unwrap(), nodes_to_data)
}
