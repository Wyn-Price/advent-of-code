use scan_fmt::scan_fmt;

use std::collections::HashSet;

struct Blueprint {
    id: u16,
    ore_cost: u16,
    clay_cost: u16,
    obsidian_ore_cost: u16,
    obsidian_clay_cost: u16,
    geode_ore_cost: u16,
    geode_obsidian_cost: u16,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct GameState {
    ore_count: u16,
    clay_count: u16,
    obsidian_count: u16,
    geodes_opened: u16,

    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
}

fn compute_quality_level(blueprint: &Blueprint, start: GameState, time: i32) -> i64 {
    let id = blueprint.id;
    println!("Starting: {id}");

    let mut next_game_states = vec![start];
    for minute in 0..time {
        let size = next_game_states.len();
        println!(" - {minute} {size}");
        let mut new_game_states = HashSet::new();

        for game in next_game_states.iter() {

            let ore_count = game.ore_count + game.ore_robots;
            let clay_count = game.clay_count + game.clay_robots;
            let obsidian_count = game.obsidian_count + game.obsidian_robots;
            let geodes_opened = game.geodes_opened + game.geode_robots;

            let ore_robots = game.ore_robots;
            let clay_robots = game.clay_robots;
            let obsidian_robots = game.obsidian_robots;
            let geode_robots = game.geode_robots;

            if game.ore_count >= blueprint.geode_ore_cost && game.obsidian_count >= blueprint.geode_obsidian_cost {
                new_game_states.insert(GameState {
                    ore_count: ore_count - blueprint.geode_ore_cost,
                    clay_count,
                    obsidian_count: obsidian_count - blueprint.geode_obsidian_cost,
                    geodes_opened,

                    ore_robots,
                    clay_robots,
                    obsidian_robots,
                    geode_robots: geode_robots + 1,
                });             
            }

            if game.ore_count >= blueprint.obsidian_ore_cost && game.clay_count >= blueprint.obsidian_clay_cost {
                new_game_states.insert(GameState {
                    ore_count: ore_count - blueprint.obsidian_ore_cost,
                    clay_count: clay_count - blueprint.obsidian_clay_cost,
                    obsidian_count,
                    geodes_opened,

                    ore_robots,
                    clay_robots,
                    obsidian_robots: obsidian_robots + 1,
                    geode_robots,
                });             
            }

            if game.ore_count >= blueprint.clay_cost {
                new_game_states.insert(GameState {
                    ore_count: ore_count - blueprint.clay_cost,
                    clay_count,
                    obsidian_count,
                    geodes_opened,

                    ore_robots,
                    clay_robots: clay_robots + 1,
                    obsidian_robots,
                    geode_robots,
                });           
            }

            if game.ore_count >= blueprint.ore_cost {
                new_game_states.insert(GameState {
                    ore_count: ore_count - blueprint.ore_cost,
                    clay_count,
                    obsidian_count,
                    geodes_opened,

                    ore_robots: ore_robots + 1,
                    clay_robots,
                    obsidian_robots,
                    geode_robots,
                });
            }
            
            new_game_states.insert(GameState {
                    ore_count,
                    clay_count,
                    obsidian_count,
                    geodes_opened,

                    ore_robots,
                    clay_robots,
                    obsidian_robots,
                    geode_robots,
            });
        }

        
        let top_geode_robots = new_game_states.iter().map(|g| g.geode_robots).max().unwrap();
        let top_geode_count = new_game_states.iter().map(|g| g.geodes_opened).max().unwrap();
        next_game_states = new_game_states.into_iter()
            .filter(|g| top_geode_robots - g.geode_robots <= 1 && top_geode_count - g.geodes_opened <= 1)
            .collect()
    }

    let max = next_game_states.into_iter().map(|g| g.geodes_opened).max().unwrap() as i64;

    println!("=> {max}");
    max
}

pub fn part_a(input: &str) -> i64 {
    let blueprints = parse(input);
    let gamestate = GameState { ore_count: 0, clay_count: 0, obsidian_count: 0, geodes_opened: 0, ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0 };
    
    blueprints.into_iter().map(|b| (b.id as i64) * compute_quality_level(&b, gamestate, 24)).sum()
}

pub fn part_b(input: &str) -> i64 {
    let blueprints = parse(input);
    let gamestate = GameState { ore_count: 0, clay_count: 0, obsidian_count: 0, geodes_opened: 0, ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0 };
    
    blueprints.into_iter().take(3).map(|b| compute_quality_level(&b, gamestate, 32)).product()
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(|l| {
        let (id, ore_cost, clay_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost) = scan_fmt!(
            l, 
            "Blueprint {d}: Each ore robot costs {d} ore. Each clay robot costs {d} ore. Each obsidian robot costs {d} ore and {d} clay. Each geode robot costs {d} ore and {d} obsidian.",
            u16, u16, u16, u16, u16, u16, u16
        ).unwrap();
        Blueprint { id, ore_cost, clay_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost }
    })
    .collect()
}
