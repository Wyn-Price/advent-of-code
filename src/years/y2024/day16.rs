use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn part_a(input: &str) -> i64 {
    let (best_score, score_to_pos) = solve_shit_shite(input);
    println!(
        "(part B is {} btw)",
        score_to_pos
            .get(&best_score)
            .map_or(0, |set| set.len() as i64 + 1)
    );
    best_score as i64
}

pub fn part_b(input: &str) -> i64 {
    let (best_score, score_to_pos) = solve_shit_shite(input);
    score_to_pos
        .get(&best_score)
        .map_or(0, |set| set.len() as i64 + 1)
}

pub fn solve_shit_shite(input: &str) -> (i32, HashMap<i32, HashSet<(i32, i32)>>) {
    let maze = parse(input);

    let mut heads = BinaryHeap::new();
    heads.push((
        0,
        maze.reindeer,
        maze.reindeer_dir,
        Vec::<(i32, i32)>::new(),
    ));
    let mut best_scores: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();
    let mut best_score = i32::MAX;
    let mut score_to_pos: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();

    while let Some((neg_score, (x, y), dir, path)) = heads.pop() {
        let score = -neg_score;
        if x == maze.end_tile.0 && y == maze.end_tile.1 {
            if score <= best_score {
                best_score = score;
                score_to_pos
                    .entry(best_score)
                    .or_insert_with(HashSet::new)
                    .extend(path);
            }
            continue;
        }

        if let Some(&best) = best_scores.get(&((x, y), dir)) {
            if best < score {
                continue;
            }
        }
        best_scores.insert(((x, y), dir), score);

        let np = (x + dir.0, y + dir.1);

        if !maze.walls.contains(&np) {
            let mut npath = path.clone();
            npath.push(np);
            heads.push((-(score + 1), np, dir, npath));
        }

        for rot_dir in vec![(dir.1.abs(), dir.0.abs()), (-dir.1.abs(), -dir.0.abs())] {
            let mut npath = path.clone();
            npath.push((x, y));
            heads.push((-(score + 1000), (x, y), rot_dir, npath));
        }
    }

    return (best_score, score_to_pos);
}

#[allow(dead_code)]
fn debug_print(maze: Maze, s: &HashSet<(i32, i32)>) {
    let h = maze.walls.iter().map(|w| w.1).max().unwrap();
    let w = maze.walls.iter().map(|w| w.0).max().unwrap();

    for y in 0..=h {
        for x in 0..=w {
            let p = (x, y);
            if s.contains(&p) {
                print!("O");
            } else if maze.walls.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

struct Maze {
    walls: HashSet<(i32, i32)>,
    end_tile: (i32, i32),
    reindeer: (i32, i32),
    reindeer_dir: (i32, i32),
}

fn parse(input: &str) -> Maze {
    let mut maze = Maze {
        walls: HashSet::new(),
        end_tile: (0, 0),
        reindeer: (0, 0),
        reindeer_dir: (1, 0), // E
    };
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let p = (x as i32, y as i32);
            match c {
                '#' => {
                    maze.walls.insert(p);
                }
                'S' => maze.reindeer = p,
                'E' => maze.end_tile = p,
                '.' => {}
                _ => panic!("Unknown char {c} at {x} {y}"),
            }
        })
    });

    maze
}
