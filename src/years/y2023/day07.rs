use std::cmp::Ordering;

type Cards = [i8; 5];

pub fn part_a(input: &str) -> i64 {
    let mut data = parse(input);
    data.sort_by(|&(a, _), &(b, _)| {
        let ta = get_type(&a);
        let tb = get_type(&b);

        if ta < tb {
            return Ordering::Less;
        }

        if ta > tb {
            return Ordering::Greater;
        }

        for c in 0..5 {
            if a[c] < b[c] {
                return Ordering::Less;
            }
            if a[c] > b[c] {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    });
    data.into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) as i32 * bid)
        .sum::<i32>() as i64
}

pub fn part_b(input: &str) -> i64 {
    let mut data: Vec<_> = parse(input)
        .into_iter()
        .map(|(cards, bid)| (get_type_b(&cards), cards, bid))
        .collect();

    data.sort_by(|&(ta, a, _), &(tb, b, _)| {
        if ta < tb {
            return Ordering::Less;
        }

        if ta > tb {
            return Ordering::Greater;
        }

        for c in 0..5 {
            let va = if a[c] == 11 { 1 } else { a[c] };
            let vb = if b[c] == 11 { 1 } else { b[c] };
            if va < vb {
                return Ordering::Less;
            }
            if va > vb {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    });
    data.into_iter()
        .enumerate()
        .map(|(idx, (_, _, bid))| (idx + 1) as i32 * bid)
        .sum::<i32>() as i64
}

fn get_type(cards: &Cards) -> i8 {
    let &[a, b, c, d, e] = cards;

    // 5
    if a == b && b == c && c == d && d == e {
        return 6;
    }

    // 4
    for &t in cards {
        let similar = cards.iter().filter(|&&c| c == t).count();
        if similar == 4 {
            return 5;
        }
    }

    // Full House
    let mut fh = true;
    for &t in cards {
        let similar = cards.iter().filter(|&&c| c == t).count();
        if similar != 3 && similar != 2 {
            fh = false;
        }
    }
    if fh {
        return 4;
    }

    // 3
    for &t in cards {
        let similar = cards.iter().filter(|&&c| c == t).count();
        if similar == 3 {
            return 3;
        }
    }

    // Two pair
    let mut num_indv_cards = 0;
    for &t in cards {
        let similar = cards.iter().filter(|&&c| c == t).count();
        if similar == 1 {
            num_indv_cards += 1;
        }
    }
    if num_indv_cards == 1 {
        return 2;
    }

    // One pair
    for &t in cards {
        let similar = cards.iter().filter(|&&c| c == t).count();
        if similar == 2 {
            return 1;
        }
    }

    // High Card
    return 0;
}

fn get_type_b(cards: &Cards) -> i8 {
    let &[a, b, c, d, e] = cards;

    // Will always be better to maximise the type, as J is worth 1
    let mut best_score = 0;
    for pa in get_poss(a) {
        for pb in get_poss(b) {
            for pc in get_poss(c) {
                for pd in get_poss(d) {
                    for pe in get_poss(e) {
                        best_score = best_score.max(get_type(&[pa, pb, pc, pd, pe]));
                    }
                }
            }
        }
    }

    best_score
}

fn get_poss(card: i8) -> Vec<i8> {
    if card == 11 {
        return vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14];
    }
    return vec![card];
}

fn parse(input: &str) -> Vec<(Cards, i32)> {
    input
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(" ").unwrap();
            let cards_vec: Vec<_> = cards
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).unwrap() as i8,
                })
                .collect();

            return (
                [
                    cards_vec[0],
                    cards_vec[1],
                    cards_vec[2],
                    cards_vec[3],
                    cards_vec[4],
                ],
                bid.parse::<i32>().unwrap(),
            );
        })
        .collect()
}
