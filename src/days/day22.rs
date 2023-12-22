use itertools::Itertools;

type Board = Vec<(i64, i64, i64, i64, i64, i64)>;

pub fn part_a(input: &str) -> i64 {
    let mut board = parse(input);
    sort_and_drop(&mut board);

    board
        .iter()
        .enumerate()
        .filter(|&(i, &(fx, fy, _, tx, ty, tz))| {
            let above = board
                .iter()
                .enumerate()
                .filter(|&(idx, &(ofx, ofy, ofz, otx, oty, _))| {
                    idx != i && tz + 1 == ofz && fx <= otx && ofx <= tx && fy <= oty && ofy <= ty
                })
                .collect_vec();

            // Check that, for each above, it's supported by something other than this cube
            let num_above_unsupported = above
                .iter()
                .filter(|&(ai, &(afx, afy, afz, atx, aty, _))| {
                    let supports = board
                        .iter()
                        .enumerate()
                        .filter(|&(idx, &(ofx, ofy, _, otx, oty, otz))| {
                            idx != i
                                && idx != *ai
                                && otz + 1 == afz
                                && afx <= otx
                                && ofx <= atx
                                && afy <= oty
                                && ofy <= aty
                        })
                        .collect_vec();
                    return supports.is_empty();
                })
                .collect_vec()
                .len();
            return num_above_unsupported == 0;
        })
        .collect_vec()
        .len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let mut board = parse(input);
    sort_and_drop(&mut board);

    board
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut copied_board = board.clone();
            copied_board.remove(i);

            loop {
                let before = copied_board.len();
                let cloned = copied_board.clone();
                copied_board.retain(|&(fx, fy, fz, tx, ty, _)| {
                    if fz == 1 {
                        return true;
                    }
                    let supports = cloned
                        .iter()
                        .copied()
                        .filter(|&(ofx, ofy, _, otx, oty, otz)| {
                            otz + 1 == fz && fx <= otx && ofx <= tx && fy <= oty && ofy <= ty
                        })
                        .collect_vec();
                    !supports.is_empty()
                });
                if copied_board.len() == before {
                    break;
                }
            }

            let other_fallen = board.len() - copied_board.len() - 1;
            return other_fallen as i64;
        })
        .sum()
}

fn sort_and_drop(board: &mut Board) {
    board.sort_by_key(|&(_, _, fz, _, _, tz)| fz.min(tz));

    for i in 0..board.len() {
        let (fx, fy, fz, tx, ty, _) = board[i];
        let max_z = board
            .iter()
            .enumerate()
            .filter(|&(idx, &(ofx, ofy, _, otx, oty, otz))| {
                idx != i && otz <= fz && fx <= otx && ofx <= tx && fy <= oty && ofy <= ty
            })
            .map(|(_, &(_, _, _, _, _, tz))| tz + 1)
            .max()
            .unwrap_or(1);

        let move_by = fz - max_z;
        board[i].2 -= move_by;
        board[i].5 -= move_by;
    }
}

fn parse(input: &str) -> Board {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("~").unwrap();

            let (fx, fy, fz) = left
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (tx, ty, tz) = right
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            (fx, fy, fz, tx, ty, tz)
        })
        .collect_vec()
}
