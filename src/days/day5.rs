use regex::Regex;

const GRID_SIZE: usize = 999;
//Gross - but done quickly
pub fn part_a(input: &str) -> i64 {
    let (mut grid, lines) = parse_grid_and_lines(input);
    for line in lines {
        if line.from.0 == line.to.0 || line.from.1 == line.to.1 {
            grid.add_line(line);
        }
    }
    grid.count_overlaps()
}

pub fn part_b(input: &str) -> i64 {
    let (mut grid, lines) = parse_grid_and_lines(input);
    for line in lines {
        if line.from.0 == line.to.0 || line.from.1 == line.to.1 {
            grid.add_line(line);
        } else {
            grid.add_diag(line);
        }
    }
    grid.count_overlaps()
}

#[derive(Debug)]
struct Grid {
    counts: [[u16; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    fn add_line(&mut self, line: Line) {
        let x1 = line.from.0.min(line.to.0);
        let x2 = line.from.0.max(line.to.0);
        let y1 = line.from.1.min(line.to.1);
        let y2 = line.from.1.max(line.to.1);
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.counts[x][y] += 1;
            }
        }
    }

    fn add_diag(&mut self, line: Line) {
        let length = (line.to.0).abs_diff(line.from.0);
        let xdir: i16 = if line.to.0 > line.from.0 { 1 } else { -1 };
        let ydir: i16 = if line.to.1 > line.from.1 { 1 } else { -1 };
        for i in 0..length + 1 {
            let x = (line.from.0 as i16 + xdir * i as i16) as usize;
            let y = (line.from.1 as i16 + ydir * i as i16) as usize;
            self.counts[x][y] += 1;
        }
    }

    fn count_overlaps(&self) -> i64 {
        let mut total = 0;
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                if self.counts[x][y] >= 2 {
                    total += 1;
                }
            }
        }

        total
    }
}

#[derive(Debug)]
struct Line {
    from: (usize, usize),
    to: (usize, usize),
}

fn parse_grid_and_lines(input: &str) -> (Grid, Vec<Line>) {
    let lines = input.lines().map(|x| parse_text_as_line(x)).collect();
    return (
        Grid {
            counts: [[0; GRID_SIZE]; GRID_SIZE],
        },
        lines,
    );
}

fn parse_text_as_line(text: &str) -> Line {
    let captures = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)")
        .unwrap()
        .captures(text)
        .unwrap();

    let values = (1..5)
        .map(|i| captures.get(i).unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (x1, y1, x2, y2) = (values[0], values[1], values[2], values[3]);

    Line {
        from: (x1, y1),
        to: (x2, y2),
    }
}
