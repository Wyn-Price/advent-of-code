use std::collections::HashMap;

use petgraph::{visit::EdgeRef, Graph};

// type Board = Graph<(i64, i64), (), Directed>;
type Board = Vec<Vec<char>>;

pub fn part_a(input: &str) -> i64 {
    let board = parse(input);

    let start_x = board[0]
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == '.')
        .unwrap()
        .0 as i64;
    let start_y = 0;

    let end_y = board.len() as i64 - 1;
    let mut queue = vec![(start_x, start_y, vec![])];
    let mut max = 0;
    while !queue.is_empty() {
        let (x, y, visited) = queue.pop().unwrap();

        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;

            if ny < 0 {
                continue;
            }

            if visited.contains(&(nx, ny)) {
                continue;
            }

            let char = board[ny as usize][nx as usize];
            match char {
                '#' => continue,
                '>' if dx != 1 => continue,
                '<' if dx != -1 => continue,
                'v' if dy != 1 => continue,
                '^' if dy != -1 => continue,
                _ => {}
            }

            if ny == end_y {
                max = max.max(visited.len() as i64 + 1);
                continue;
            }

            let mut nv = visited.clone();
            nv.push((x, y));

            queue.push((nx, ny, nv))
        }
    }

    max
}

pub fn part_b(input: &str) -> i64 {
    let board = parse(input);

    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();

    let start_x = board[0]
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == '.')
        .unwrap()
        .0 as i64;
    let start_y = 0;

    let end_y = board.len() as i64 - 1;
    let end_x = board[end_y as usize]
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == '.')
        .unwrap()
        .0 as i64;

    let start_node = graph.add_node(0);
    nodes.insert((start_x, start_y), start_node);

    let end_node = graph.add_node(0);
    nodes.insert((end_x, end_y), end_node);
    let mut queue = vec![(start_x, start_y, start_x, start_y, 0i64, vec![])];

    while !queue.is_empty() {
        let (origin_x, origin_y, x, y, steps, visited) = queue.pop().unwrap();

        let mut nv = visited.clone();
        nv.push((x, y));

        if origin_x != x || origin_y != y {
            if let Some(&node) = nodes.get(&(x, y)) {
                let &from = nodes.get(&(origin_x, origin_y)).unwrap();
                graph.add_edge(from, node, steps);
                continue;
            }
        }

        let mut next = vec![];

        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;

            if ny < start_y || ny > end_y {
                continue;
            }

            if visited.contains(&(nx, ny)) {
                continue;
            }

            if board[ny as usize][nx as usize] == '#' {
                continue;
            }

            next.push((nx, ny))
        }

        if next.is_empty() {
            continue;
        }

        if next.len() == 1 {
            let (nx, ny) = next[0];
            queue.push((origin_x, origin_y, nx, ny, steps + 1, nv))
        } else {
            let node = graph.add_node(0);
            nodes.insert((x, y), node);

            let &from = nodes.get(&(origin_x, origin_y)).unwrap();

            graph.add_edge(from, node, steps);

            for (nx, ny) in next {
                queue.push((x, y, nx, ny, 1, nv.clone()));
            }
        }
    }

    let mut queue = vec![(start_node, 0, vec![])];

    let mut max = 0;
    while let Some((node, steps, visited)) = queue.pop() {
        for edge in graph.edges(node) {
            let n = edge.target();
            if visited.contains(&n) {
                continue;
            }

            if n == end_node {
                max = max.max(steps + edge.weight());
                continue;
            }

            let mut nv = visited.clone();
            nv.push(n);
            queue.push((n, steps + edge.weight(), nv))
        }
    }

    max
}

fn parse(input: &str) -> Board {
    input.lines().map(|l| l.chars().collect()).collect()
}
