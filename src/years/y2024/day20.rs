use std::collections::{HashMap, HashSet};

use petgraph::{
    algo::{astar, dijkstra},
    Directed, Graph,
};

type TrackGraph = Graph<(i32, i32), i32, Directed>;
type TrackNodes = HashMap<(i32, i32), petgraph::prelude::NodeIndex>;

pub fn part_a(input: &str) -> i32 {
    let track = parse(input);
    solve(track, 2)
}

pub fn part_b(input: &str) -> i32 {
    let track = parse(input);
    solve(track, 20)
}

fn solve(track: Racetrack, c: i32) -> i32 {
    let (graph, nodes) = to_graph(&track);
    let dists = dijkstra(&graph, nodes[&track.start], None, |_| 1);
    let (_, path) = astar(
        &graph,
        nodes[&track.start],
        |n| n == nodes[&track.end],
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();
    let rev_nodes = nodes
        .iter()
        .map(|(&k, &v)| (v, k))
        .collect::<HashMap<_, _>>();

    let cheat_paths = get_paths_for(c);

    path.into_iter()
        .map(|n| rev_nodes[&n])
        .filter_map(|(x, y)| {
            let dist = *dists.get(nodes.get(&(x, y))?)?;
            let saved = cheat_paths
                .iter()
                .filter_map(|(mx, my, length)| {
                    let np = (x + mx, y + my);
                    let other_dist = *dists.get(nodes.get(&np)?)?;
                    Some(other_dist - dist - length)
                })
                .filter(|&d| d >= 100)
                .count() as i32;
            Some(saved)
        })
        .sum()
}

fn get_paths_for(cheat_dist: i32) -> Vec<(i32, i32, i32)> {
    (-cheat_dist..=cheat_dist)
        .flat_map(|x| {
            let l = cheat_dist - x.abs();
            return (-l..=l).map(move |y| (x, y, x.abs() + y.abs()));
        })
        .collect()
}

fn to_graph(track: &Racetrack) -> (TrackGraph, TrackNodes) {
    let mut graph = Graph::new();
    let mut nodes = HashMap::new();
    let w = track.walls.iter().map(|w| w.0).max().unwrap() + 1;
    let h = track.walls.iter().map(|w| w.1).max().unwrap() + 1;
    for x in 0..w {
        for y in 0..h {
            if track.walls.contains(&(x, y)) {
                continue;
            }
            let node = *nodes
                .entry((x, y))
                .or_insert_with(|| graph.add_node((x, y)));
            for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let np = (x + dx, y + dy);
                if track.walls.contains(&np) {
                    continue;
                }
                let other_node = *nodes.entry(np).or_insert_with(|| graph.add_node(np));
                graph.add_edge(node, other_node, 1);
                graph.add_edge(other_node, node, 1);
            }
        }
    }
    return (graph, nodes);
}

struct Racetrack {
    walls: HashSet<(i32, i32)>,
    start: (i32, i32),
    end: (i32, i32),
}

fn parse(input: &str) -> Racetrack {
    let mut maze = Racetrack {
        walls: HashSet::new(),
        end: (0, 0),
        start: (0, 0),
    };
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let p = (x as i32, y as i32);
            match c {
                '#' => {
                    maze.walls.insert(p);
                }
                'S' => maze.start = p,
                'E' => maze.end = p,
                '.' => {}
                _ => panic!("Unknown char {c} at {x} {y}"),
            }
        })
    });

    maze
}
