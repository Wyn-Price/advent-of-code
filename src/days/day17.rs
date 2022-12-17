use std::collections::HashMap;

const SHAPES: [Shape; 5] = [
    [[true; 4], [false; 4], [false; 4], [false; 4]],
    [
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
        [false; 4],
    ],
    [
        [false, false, true, false],
        [false, false, true, false],
        [true, true, true, false],
        [false; 4],
    ],
    [
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ],
    [
        [true, true, false, false],
        [true, true, false, false],
        [false; 4],
        [false; 4],
    ],
];

fn shape_height(shape: &Shape) -> usize {
    for ry in 0..4 {
        let y = 3 - ry;
        if shape[y].iter().any(|s| *s) {
            return y + 1;
        }
    }
    panic!("Shape is empty?")
}

type Shape = [[bool; 4]; 4];
type Space = Vec<Vec<bool>>;

struct ShapeInstance {
    shape: Shape,
    x: usize,
    y: usize,
}

impl ShapeInstance {
    fn can_sit(&self, space: &Space) -> bool {
        for vy in 0..4 {
            for vx in 0..4 {
                let x = self.x + vx;
                let y = self.y + vy;

                if self.shape[vy][vx] && (y >= space.len() || x >= space[y].len() || space[y][x]) {
                    return false;
                }
            }
        }
        true
    }

    fn move_if_possible(&mut self, space: &Space, x: i32, y: i32) -> bool {
        let sx = self.x;
        let sy = self.y;

        let mx = self.x as i32 + x;
        if mx < 0 {
            return false;
        }

        self.x = mx as usize;
        self.y = (self.y as i32 + y) as usize;

        let can_sit = self.can_sit(space);

        if !can_sit {
            self.x = sx;
            self.y = sy;
        }

        return can_sit;
    }
}

pub fn drop_all_rocks<const ROUNDS: usize>(gas: Vec<bool>) -> i64 {
    let mut gas_index = 0;

    //As soon as we've seen this state before:
    // a
    // b
    // c
    // ..
    // l
    // b - seen before, we can repeat b -> l as many times as possible
    //

    //We need to know what the index of the rock was to know how many times to repeat
    //We need to know how high the rock was, to know how much it's changed and how much to add

    // shape_index, gas_index, [top_spaces];
    type Key = (usize, usize, [isize; 7]);

    // rock_index, rock_height
    type Value = (usize, usize);

    let mut cache: HashMap<Key, Value> = HashMap::new();

    let window_size = 100;
    let mut window_offset = 0;

    let mut space = vec![vec![false; 7]; window_size];

    let mut top_y = window_size;

    let mut r = 0;
    while r < ROUNDS {
        let shape_index = r % SHAPES.len();
        drop_rock(
            r,
            &gas,
            &mut gas_index,
            &mut space,
            &mut top_y,
            &mut window_offset,
        );

        let mut tops = [0_usize; 7];
        for x in 0..7 {
            for y in 0..window_size {
                if space[y][x] {
                    tops[x] = y;
                    break;
                }
            }
        }

        r += 1;

        let normalized = tops.map(|t| if t == 0 { -1 } else { (t - top_y) as isize });

        let key: Key = (shape_index, gas_index.clone(), normalized);
        let val: Value = (r, window_offset + space.len() - top_y);

        if let Some((other_index, other_height)) = cache.insert(key, val) {
            println!("FOUND LOOP FROM {other_index} TO {r}");
            let (my_index, my_height) = val;

            let repeat_size = my_index - other_index;
            let repeat_height = my_height - other_height;

            let rounds_left = ROUNDS - my_index;
            let repeat = rounds_left / repeat_size;

            let height_addition = repeat_height * repeat;
            let round_start = my_index + repeat_size * repeat;

            for r in round_start..ROUNDS {
                drop_rock(
                    r,
                    &gas,
                    &mut gas_index,
                    &mut space,
                    &mut top_y,
                    &mut window_offset,
                );
            }

            return (window_offset + space.len() - top_y + height_addition) as i64;
        }
    }

    (window_offset + space.len() - top_y) as i64
}

fn drop_rock(
    index: usize,
    gas: &Vec<bool>,
    gas_index: &mut usize,
    space: &mut Vec<Vec<bool>>,
    top_y: &mut usize,
    window_offset: &mut usize,
) {
    let shape = SHAPES[index % SHAPES.len()];
    let mut s = ShapeInstance {
        shape,
        x: 2,
        y: *top_y - 3 - shape_height(&shape),
    };
    loop {
        let g = gas[*gas_index];
        *gas_index = (*gas_index + 1) % gas.len();
        let d = if g { 1 } else { -1 };
        s.move_if_possible(&*space, d, 0);

        if !s.move_if_possible(&*space, 0, 1) {
            break;
        }
    }
    for y in 0..4 {
        for x in 0..4 {
            if s.shape[y][x] {
                space[y + s.y][x + s.x] = true;
                *top_y = *top_y.min(&mut (y + s.y));
            }
        }
    }
    while *top_y <= 20 {
        space.pop();
        space.insert(0, vec![false; 7]);
        *window_offset += 1;
        *top_y += 1;
    }
}

pub fn part_a(input: &str) -> i64 {
    let gas = parse(input);
    drop_all_rocks::<2022>(gas)
}

pub fn part_b(input: &str) -> i64 {
    let gas = parse(input);
    drop_all_rocks::<1000000000000>(gas)
}

fn parse(input: &str) -> Vec<bool> {
    input.trim().chars().map(|c| c == '>').collect()
}
