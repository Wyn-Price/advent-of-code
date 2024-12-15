use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
    start: (i32, i32),
    end: (i32, i32),
    delta: (i32, i32),
}

pub fn part_a(input: &str) -> i64 {
    let grids = parse(input);
    grids
        .into_iter()
        .flat_map(|(_, positions)| get_edges_and_areas(positions))
        .map(|(seen, edges)| seen.len() as i64 * edges.len() as i64)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let grids = parse(input);
    grids
        .into_iter()
        .flat_map(|(_, positions)| get_edges_and_areas(positions))
        .map(|(seen, mut edges)| {
            'outer: loop {
                for e1 in 0..edges.len() {
                    for e2 in 0..edges.len() {
                        if e1 == e2 {
                            continue;
                        }

                        let edge1 = edges[e1];
                        let edge2 = edges[e2];

                        let can_combine = edge1.end == edge2.start && edge1.delta == edge2.delta;
                        if can_combine {
                            edges.remove(e1.max(e2));
                            edges.remove(e1.min(e2));
                            edges.push(Edge {
                                start: edge1.start,
                                end: edge2.end,
                                delta: edge1.delta,
                            });
                            continue 'outer;
                        }
                    }
                }
                break;
            }
            (seen, edges)
        })
        .map(|(seen, edges)| seen.len() as i64 * edges.len() as i64)
        .sum()
}

fn get_edges_and_areas(positions: Vec<(i32, i32)>) -> Vec<(HashSet<(i32, i32)>, Vec<Edge>)> {
    let mut positions_left = positions.clone();
    let mut ret = vec![];
    while !positions_left.is_empty() {
        let mut head = vec![positions_left[0]];
        let mut seen = HashSet::new();
        let mut edges = vec![];
        while !head.is_empty() {
            let (px, py) = head.remove(0);
            if !seen.insert((px, py)) {
                continue;
            }
            positions_left.retain(|&(ex, ey)| ex != px || ey != py);
            for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let np = (px + dx, py + dy);
                if positions.contains(&np) {
                    head.push(np);
                } else {
                    edges.push(Edge {
                        start: (px, py),
                        end: (px + dy, py + dx),
                        delta: (dy, dx),
                    });
                }
            }
        }
        ret.push((seen, edges));
    }
    ret
}

fn parse(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .map(move |(x, c)| (c, (x as i32, y as i32)))
        })
        .fold(HashMap::new(), |mut map, (c, v)| {
            map.entry(c)
                .and_modify(|vec| vec.push(v))
                .or_insert(vec![v]);
            map
        })
}
