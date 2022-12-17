use std::ffi::FromVecWithNulError;

const shapes: [Shape; 5] = [
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

pub fn part_a(input: &str) -> i64 {
    let gas = parse(input);
    let mut gas_index = 0;
    let mut next_gas = || {
        let g = gas[gas_index % gas.len()];
        gas_index += 1;
        return g;
    };

    let rounds = 2022;

    let mut space = vec![vec![false; 7]; 4 * rounds + 4];

    let mut top_y = space.len();

    for r in 0..rounds {
        let shape = shapes[r % shapes.len()];
        let mut s = ShapeInstance {
            shape,
            x: 2,
            y: top_y - 3 - shape_height(&shape),
        };

        // println!("{r}: ");
        // let h = shape_height(&shape);
        // println!(" - {h}");
        // debug_print(&space, &s);
        loop {
            // println!("V");
            // debug_print(&space, &s);

            let g = next_gas();
            let d = if g { 1 } else { -1 };
            s.move_if_possible(&space, d, 0);

            // if g {
            //     println!(">");
            // } else {
            //     println!("<");
            // }

            if !s.move_if_possible(&space, 0, 1) {
                // debug_print(&space, &s);
                break;
            }

            // debug_print(&space, &s);
        }

        for y in 0..4 {
            for x in 0..4 {
                if s.shape[y][x] {
                    space[y + s.y][x + s.x] = true;
                    top_y = top_y.min(y + s.y);
                }
            }
        }
    }

    dbg!(top_y, space.len());
    (space.len() - top_y) as i64
}

fn debug_print(space: &Vec<Vec<bool>>, s: &ShapeInstance) {
    for y in 0..space.len() {
        for x in 0..7 {
            if y >= s.y && x >= s.x {
                let sy = y - s.y;
                let sx = x - s.x;

                if sy < 4 && sx < 4 {
                    if s.shape[sy][sx] {
                        print!("@");
                        continue;
                    }
                }
            }

            if space[y][x] {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

pub fn part_b(input: &str) -> i64 {
    panic!("Part B not implimented yet");
}

fn parse(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '>').collect()
}
