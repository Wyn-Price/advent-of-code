use std::collections::HashSet;

type Dir = (i8, i8);
type Pos = (i64, i64);

const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);
const LEFT: Dir = (-1, 0);
const RIGHT: Dir = (1, 0);
const NONE: Dir = (0, 0);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Blizzard {
    position: Pos,
    direction: Dir
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct GameState {
    position: Pos,
    // blizzards: Vec<Blizzard>
}

#[derive(PartialEq)]
enum State {
    First,
    GoBackForSnacks,
    ComeBackWithSnacks,
}

fn compute(input: &str, break_on_state: State) -> i64 {

    let (mut blizzards, width, height, start, end) = parse(input);

    let mut game_states = vec![GameState{ position: start }];
    let mut begin = start;
    let mut target = end;

    let mut round = 0;
    let mut state = State::First;

    loop {
        let mut finished = false;
        round += 1;
        if round % 50 == 0 {
            println!("{} - {}", round, game_states.len());
        }
        if game_states.is_empty() {
            panic!("Empty states");
        }
        let mut next_blizzards = vec![];
        for b in blizzards {
            next_blizzards.push(Blizzard {
                position: (
                    (b.position.0 + b.direction.0 as i64 + width) % width,
                    (b.position.1 + b.direction.1 as i64 + height) % height,
                ),
                direction: b.direction
            });
        }
        blizzards = next_blizzards;

        let mut new_game_states = HashSet::new();
        for game in &game_states {
            for dir in vec![DOWN, RIGHT, UP, LEFT, NONE] {
                let new_pos = (
                    game.position.0 + dir.0 as i64,
                    game.position.1 + dir.1 as i64,
                );
                if target.eq(&new_pos) {
                    finished = true;
                    break;
                }
                if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= width || new_pos.1 >= height ||
                        blizzards.iter().any(|b| new_pos.eq(&b.position)) {
                    // Allow the start pos
                    if !new_pos.eq(&begin) {
                        continue;
                    }
                   }

                new_game_states.insert(GameState {
                    position: new_pos,
                });
            }
        }

        game_states = new_game_states.into_iter().collect();

        if finished {
            if break_on_state == state {
                break
            }
            match state {
                State::First => {
                    state = State::GoBackForSnacks;
                    game_states = vec![GameState{ position: end }];
                    begin = end;
                    target = start;
                },
                State::GoBackForSnacks => {
                    state = State::ComeBackWithSnacks;
                    game_states = vec![GameState{ position: start }];
                    begin = start;
                    target = end;
                },
                _ => {},
            }
        }
    }

    round
}

pub fn part_a(input: &str) -> i64 {
    compute(input, State::First)
}

pub fn part_b(input: &str) -> i64 {
    compute(input, State::ComeBackWithSnacks)
}

fn parse(input: &str) -> (Vec<Blizzard>, i64, i64, Pos, Pos) {
    let vec = input.lines().enumerate().flat_map(|(y, l)| l.char_indices().filter_map(move |(x, c)| {
        let dir: Dir = match c {
            '>' => Some((1, 0)),
            '<' => Some((-1, 0)),
            'v' => Some((0, 1)),
            '^' => Some((0, -1)),
            _ => None
        }?;
        Some(Blizzard { position: (x as i64 - 1, y as i64 - 1), direction: dir})
    })).collect::<Vec<_>>();

    let lines_vec = input.trim().split("\n").collect::<Vec<_>>();
    let width = lines_vec[0].len() as i64 - 2;
    let height = lines_vec.len()as i64 - 2;

    let start: Pos = (0, -1);
    let end: Pos = (width - 1, height);

    return (vec, width, height, start, end)
}
