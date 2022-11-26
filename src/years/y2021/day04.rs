const GRID_SIZE: usize = 5;

pub fn part_a(input: &str) -> i64 {
    let (commands, mut boards) = parse_input(input);

    for number in commands {
        for board in boards.iter_mut() {
            board.number_called(number);

            if board.has_won() {
                return board.calc_score() * number as i64;
            }
        }
    }

    panic!("Nobody won?!");
}

pub fn part_b(input: &str) -> i64 {
    let (commands, mut boards) = parse_input(input);

    let mut count = boards.len();
    for number in commands {
        for board in &mut boards {
            board.number_called(number);

            if board.has_won() {
                count -= 1;
                if count == 0 {
                    return board.calc_score() * number as i64;
                }
            }
        }

        boards.retain(|b| !b.has_won());
    }

    panic!("Nobody won?!");
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let (cmdstr, boards) = input.split_once("\n\n").unwrap();

    (
        cmdstr.split(',').map(|c| c.parse().unwrap()).collect(),
        boards
            .split("\n\n")
            .map(|s| {
                let row_numbers: [[usize; 5]; 5] = s
                    .split_ascii_whitespace()
                    .map(|d| d.parse().unwrap())
                    .array_chunks::<5>()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                return BingoBoard {
                    grid: row_numbers,
                    marked: [[false; 5]; 5],
                };
            })
            .collect(),
    )
}
struct BingoBoard {
    grid: [[usize; GRID_SIZE]; GRID_SIZE],
    marked: [[bool; GRID_SIZE]; GRID_SIZE],
}

// Gross - nicer would be to store marked as a single u32 number, and have the grid be a map of number -> position
impl BingoBoard {
    fn number_called(&mut self, number: usize) {
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                if self.grid[row][column] == number {
                    self.marked[row][column] = true;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        for row in 0..GRID_SIZE {
            let mut full = true;
            for col in 0..GRID_SIZE {
                if !self.marked[row][col] {
                    full = false;
                }
            }
            if full {
                return true;
            }
        }

        for col in 0..GRID_SIZE {
            let mut full = true;
            for row in 0..GRID_SIZE {
                if !self.marked[row][col] {
                    full = false;
                }
            }
            if full {
                return true;
            }
        }

        false
    }

    fn calc_score(&self) -> i64 {
        let mut sum: usize = 0;

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if !self.marked[row][col] {
                    sum += self.grid[row][col];
                }
            }
        }

        sum.try_into().unwrap()
    }
}
