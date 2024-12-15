use std::collections::{HashMap, HashSet};

type Pos = (u8, u8, u8);
type Positions = HashSet<Pos>;
type Cache = HashMap<Pos, bool>;

const OFFSETS: [(i8, i8, i8); 6] = [
    // Up should go first, down should go last
    (0, 1, 0),
    (1, 0, 0),
    (-1, 0, 0),
    (0, 0, 1),
    (0, 0, -1),
    (0, -1, 0),
];

pub fn part_a(input: &str) -> i64 {
    let cubes: HashSet<Pos> = HashSet::from_iter(parse(input).into_iter());

    let mut total_sides = 0;

    for cube in cubes.iter() {
        for offset in OFFSETS {
            let newpos = (
                (cube.0 as i8 + offset.0) as u8,
                (cube.1 as i8 + offset.1) as u8,
                (cube.2 as i8 + offset.2) as u8,
            );
            if !cubes.contains(&newpos) {
                total_sides += 1;
            }
        }
    }

    total_sides
}

pub fn part_b(input: &str) -> i64 {
    let cubes: Positions = HashSet::from_iter(parse(input).into_iter());

    let mut adjacent_air = HashSet::new();
    for cube in cubes.iter() {
        for offset in OFFSETS {
            let newpos = (
                (cube.0 as i8 + offset.0) as u8,
                (cube.1 as i8 + offset.1) as u8,
                (cube.2 as i8 + offset.2) as u8,
            );
            if !cubes.contains(&newpos) {
                adjacent_air.insert(newpos);
            }
        }
    }

    let y_limit = cubes.iter().max_by_key(|c| c.1).unwrap().1 + 2;

    let mut computed_air: Cache = HashMap::new();
    for air in adjacent_air {
        let mut computed = HashSet::new();
        let inside = air_inside(air, &cubes, &mut computed, &computed_air, y_limit);
        for air in computed {
            computed_air.insert(air, inside);
        }
    }

    let mut total_sides = 0;
    for cube in cubes.iter() {
        for offset in OFFSETS {
            let newpos = (
                (cube.0 as i8 + offset.0) as u8,
                (cube.1 as i8 + offset.1) as u8,
                (cube.2 as i8 + offset.2) as u8,
            );
            if !cubes.contains(&newpos) && *computed_air.get(&newpos).unwrap() {
                total_sides += 1;
            }
        }
    }
    total_sides
}

fn air_inside(
    pos: Pos,
    cubes: &Positions,
    computed: &mut Positions,
    cache: &Cache,
    y_limit: u8,
) -> bool {
    if let Some(result) = cache.get(&pos) {
        return *result;
    }

    computed.insert(pos);
    for offset in OFFSETS {
        let newpos = (
            (pos.0 as i8 + offset.0) as u8,
            (pos.1 as i8 + offset.1) as u8,
            (pos.2 as i8 + offset.2) as u8,
        );

        if cubes.contains(&newpos) {
            continue;
        }

        if newpos.1 >= y_limit {
            return true;
        }

        if !computed.contains(&newpos) {
            if air_inside(newpos, cubes, computed, cache, y_limit) {
                return true;
            }
        }
    }

    false
}

fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let data: Vec<_> = l.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
            (data[0], data[1], data[2])
        })
        .collect()
}
