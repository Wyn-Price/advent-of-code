use std::{
    borrow::{Borrow, BorrowMut},
    collections::{HashMap, HashSet},
};

use petgraph::{algo::dijkstra, data::Build, stable_graph::NodeIndex, Graph};

pub fn part_a(input: &str) -> i64 {
    let (mut map, graph) = parse(input);
    // let best = find_best(-1, 1, "AA".to_owned(), 0, HashSet::new(), &mut map);
    // best

    for (key, node) in map.iter_mut() {
        let node_map = dijkstra(&graph, node.node, None, |_| 1);

        let time_to_nodes = map
            .values()
            .filter(|&v| !v.key.starts_with("AA"))
            .map(|v| (v.key.to_owned(), *node_map.get(&v.node).unwrap() as i64))
            .collect::<Vec<_>>();

        node.time_to_nodes = time_to_nodes;
    }

    find_best(1, "AA".to_owned(), 0, 0, HashSet::new(), &map, &graph)

    // let mut current_node = "AA".to_owned();
    // let mut opened_valves = HashSet::new();

    // let mut total_rate: i64 = 0;
    // let mut score: i64 = 0;

    // let mut time = 1;

    // while time < 30 {
    //     let current = map.get(&current_node).unwrap();
    //     let node_map = dijkstra(&graph, current.node, None, |_| 1);

    //     let time_to_nodes = map
    //         .values()
    //         .filter(|&v| !opened_valves.contains(&v.key))
    //         .map(|v| (v, *node_map.get(&v.node).unwrap() as i64))
    //         .collect::<Vec<_>>();

    //     // dbg!(&time_to_nodes);

    //     let (best_node, mut time_taken) = time_to_nodes
    //         .into_iter()
    //         .max_by_key(|&(node, time_it_takes)| node.rate as i64 - time_it_takes)
    //         .unwrap();

    //     if time + time_taken >= 30 {
    //         time_taken = 30 - time - 1;
    //     }

    //     // dbg!(&best_node, &time_taken);
    //     // The +1 is to open the valve
    //     print!("{time}");
    //     time += time_taken + 1;

    //     let k = &best_node.key;
    //     println!(" - {time} ({k} {total_rate})");

    //     score += (time_taken + 1) * total_rate;

    //     current_node = best_node.key.clone();
    //     opened_valves.insert(best_node.key.clone());
    //     total_rate += best_node.rate as i64;
    // }

    // score
}

fn find_best(
    time: i64,
    game_current: String,
    game_total_rate: i64,
    game_score: i64,
    game_open_valves: HashSet<String>,
    valves: &HashMap<String, Valve>,
    graph: &Graph<String, ()>,
) -> i64 {
    let current = valves.get(&game_current).unwrap();

    current
        .time_to_nodes
        .iter()
        .map(|(valve_key, mut time_taken)| {
            let valve = valves.get(valve_key).unwrap();
            if time + time_taken >= 30 {
                time_taken = 30 - time - 1;
            }

            let new_game_time = time + time_taken + 1;
            let new_game_score = game_score + (time_taken + 1) * game_total_rate;

            if time == 30 {
                return new_game_score;
            }

            let new_game_current = valve.key.clone();

            let mut new_game_open_valves = game_open_valves.clone();
            new_game_open_valves.insert(valve.key.clone());

            let new_game_total_rate = game_total_rate + valve.rate as i64;

            // println!("{valve:?} {time_taken}");
            // new_game_score

            find_best(
                new_game_time,
                new_game_current,
                new_game_total_rate,
                new_game_score,
                new_game_open_valves,
                valves,
                graph,
            )
        })
        .max()
        .unwrap_or_else(|| game_score + (31 - time) * game_total_rate)
}

pub fn part_b(input: &str) -> i64 {
    parse(input);
    panic!("Part B not implimented yet");
}

// fn find_best(
//     lookahead: i8,
//     minute: u8,
//     game_current: String,
//     game_score_lastround: i64,
//     game_open_valves: HashSet<String>,
//     valves: &mut HashMap<String, Valve>,
// ) -> i64 {
//     let game_score = game_score_lastround
//         + game_open_valves
//             .iter()
//             .map(|v| valves.get(v).unwrap().rate as i64)
//             .sum::<i64>();

//     if minute == 30 || lookahead == 0 {
//         return game_score;
//     }

//     let node = valves.get_mut(&game_current).unwrap();

//     let mut choices = vec![];

//     if !game_open_valves.contains(&game_current) {
//         choices.push(Move::OpenValve);
//     }

//     for state in &node.leads_to {
//         choices.push(Move::MoveTo(state.clone()))
//     }

//     let mut compute_move = |lookahead, m| {
//         let _a: &Move = m;
//         match m {
//             Move::OpenValve => {
//                 let mut open_valves = game_open_valves.clone();
//                 open_valves.insert(game_current.clone());
//                 find_best(
//                     lookahead,
//                     minute + 1,
//                     game_current.clone(),
//                     game_score,
//                     open_valves,
//                     valves,
//                 )
//             }
//             Move::MoveTo(state) => find_best(
//                 lookahead,
//                 minute + 1,
//                 state.clone(),
//                 game_score_lastround,
//                 game_open_valves.clone(),
//                 valves,
//             ),
//             Move::Nothing => game_score,
//         }
//     };

//     let lh = if lookahead == -1 { 10 } else { lookahead - 1 };

//     let (best_score, best_move) = choices
//         .iter()
//         .map(|c| {
//             let score = compute_move(lh, c);
//             return (score, c);
//         })
//         .max_by_key(|&(score, _)| score)
//         .unwrap();

//     if lookahead == -1 {
//         return compute_move(-1, &best_move);
//     } else {
//         return best_score;
//     }

//     // 0

//     // A choice will always be to do nothing
//     // let mut choices = vec![game_score];

//     // if !game_open_valves.contains(&game_current) {
//     // }

//     // let leads_to = &valves.get(&game_current).unwrap().leads_to.clone();

//     // for state in leads_to {
//     // }

//     // let l = choices.len();
//     // println!("{minute} {l}");

//     // choices.into_iter().max().unwrap()
// }

fn parse(input: &str) -> (HashMap<String, Valve>, Graph<String, ()>) {
    let mut graph = Graph::new();
    let map: HashMap<String, Valve> = input
        .lines()
        .map(|l| {
            let (key, rate, t, l, v, leading_to) = scan_fmt!(
                l,
                r"Valve {} has flow rate={d}; {} {} to {} {/.+/}",
                String,
                u8,
                String,
                String,
                String,
                String
            )
            .unwrap();

            let node = graph.add_node(key.clone());

            (
                key.clone(),
                Valve {
                    key,
                    node,
                    rate,
                    leads_to: leading_to.split(", ").map(|s| s.to_owned()).collect(),
                    time_to_nodes: vec![],
                },
            )
        })
        .collect();

    map.values().for_each(|v| {
        v.leads_to
            .iter()
            .map(|l| map.get(l).unwrap().node)
            .for_each(|other| {
                graph.add_edge(v.node, other, ());
            })
    });

    (map, graph)
}
#[derive(Debug)]
struct Valve {
    key: String,
    node: NodeIndex,
    rate: u8,
    leads_to: Vec<String>,
    time_to_nodes: Vec<(String, i64)>,
}
