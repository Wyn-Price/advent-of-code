use std::vec;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let mut data = parse(input);

    loop {
        let free_spaces = data
            .iter()
            .enumerate()
            .filter(|(_, &ref d)| d.free > 0)
            .map(|(idx, _)| idx)
            .collect_vec();

        if free_spaces.is_empty() {
            break;
        }

        let mut inserted = 0;
        for idx_old in free_spaces {
            let idx = idx_old + inserted;

            let lidx = data.len() - 1;

            // If the last has no more data remove it
            if data[lidx].taken <= 0 {
                data.pop().unwrap();
                break;
            }

            // Take away the blocks from the last
            let can_take = data[idx].free.min(data[lidx].taken);
            data[lidx].taken -= can_take;

            data.insert(
                idx + 1,
                Disk {
                    id: data[lidx].id,
                    taken: can_take,
                    free: data[idx].free - can_take,
                },
            );

            // No new free space after this one, as we've just inserted
            data[idx].free = 0;
            inserted += 1;
        }
    }

    checksum(data)
}

pub fn part_b(input: &str) -> i64 {
    let mut data = parse(input);

    let mut ptr = data.len() - 1;

    while ptr > 0 {
        let lidx = ptr;

        let free_spaces = data
            .iter()
            .enumerate()
            .filter(|(idx, &ref d)| *idx < lidx && d.free >= data[lidx].taken)
            .map(|(idx, _)| idx)
            .collect_vec();

        if free_spaces.is_empty() {
            ptr -= 1;
            continue;
        }

        let last = data.remove(lidx);
        // Add on the free space to the previous
        data[lidx - 1].free += last.taken + last.free;
        let idx = free_spaces[0];
        let can_take = last.taken;
        data.insert(
            idx + 1,
            Disk {
                id: last.id,
                taken: can_take,
                free: data[idx].free - can_take,
            },
        );

        // No new free space after this one, as we've just inserted
        data[idx].free = 0;
    }

    checksum(data)
}

fn checksum(data: Vec<Disk>) -> i64 {
    data.into_iter()
        .flat_map(|d| {
            vec![
                vec![d.id].repeat(d.taken as usize),
                vec![0].repeat(d.free as usize),
            ]
            .concat()
        })
        .enumerate()
        .map(|(idx, id)| idx as i64 * id as i64)
        .sum()
}

#[derive(Debug)]
struct Disk {
    id: i32,
    taken: i32,
    free: i32,
}

fn parse(input: &str) -> Vec<Disk> {
    let mut nums = input
        .chars()
        .filter_map(|c| c.to_string().parse::<i32>().ok())
        .collect_vec();

    let mut vec = vec![];

    while !nums.is_empty() {
        let a = nums.remove(0);
        let b = if let Some(_) = nums.get(0) {
            nums.remove(0)
        } else {
            0 as i32
        };
        vec.push((a, b));
    }

    vec.into_iter()
        .enumerate()
        .map(|(id, (taken, free))| Disk {
            free,
            id: id as i32,
            taken,
        })
        .collect_vec()
}
