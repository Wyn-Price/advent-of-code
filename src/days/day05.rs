use std::ops::Range;

#[derive(Debug)]
struct ParsedMap(Vec<(i64, i64, i64)>);

#[derive(Debug)]
struct ParsedData {
    seeds: Vec<Range<i64>>,
    maps: Vec<ParsedMap>,
}

impl ParsedMap {
    fn get_output(&self, r: Range<i64>) -> Vec<Range<i64>> {
        for &(src, dest, len) in &self.0 {
            let src_start = src;
            let src_end = src + len;

            if src_start < r.end && r.start < src_end {
                let mut split_ranges = vec![];

                let intersect_start = r.start.max(src_start);
                let intersect_end = r.end.min(src_end);

                let from = dest + (intersect_start - src_start);
                let to = dest + (intersect_end - src_start);

                split_ranges.push(from..to);

                if intersect_start > r.start {
                    split_ranges.extend(self.get_output(r.start..intersect_start))
                }

                if intersect_end < r.end {
                    split_ranges.extend(self.get_output(intersect_end..r.end))
                }

                return split_ranges;
            }
        }
        return vec![r];
    }
}

pub fn part_a(input: &str) -> i64 {
    let data = parse(input, true);
    let mut seeds = data.seeds;
    for map in data.maps {
        seeds = seeds.into_iter().flat_map(|s| map.get_output(s)).collect();
    }

    seeds.into_iter().map(|r| r.start).min().unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let data = parse(input, false);
    let mut seeds = data.seeds;
    for map in data.maps {
        seeds = seeds.into_iter().flat_map(|s| map.get_output(s)).collect();
    }

    seeds.into_iter().map(|r| r.start).min().unwrap()
}

fn parse(input: &str, part_a: bool) -> ParsedData {
    let mut seeds = vec![];
    let mut all_maps = vec![];

    input.split("\n\n").for_each(|seq| {
        let (cmd, data) = seq.split_once(':').unwrap();
        if cmd == "seeds" {
            if part_a {
                seeds = data
                    .trim()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .map(|n| n..n + 1)
                    .collect();
            } else {
                let pairs = data
                    .trim()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                for i in 0..pairs.len() / 2 {
                    let s = pairs[2 * i];
                    let r = pairs[2 * i + 1];
                    seeds.push(s..s + r + 1)
                }
            }
            return;
        }

        let data = data
            .trim()
            .lines()
            .map(|line| {
                let data = line
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                let dest = data[0];
                let src = data[1];
                let len = data[2];

                (src, dest, len)
            })
            .collect();
        all_maps.push(ParsedMap(data));
    });
    ParsedData {
        seeds,
        maps: all_maps,
    }
}
