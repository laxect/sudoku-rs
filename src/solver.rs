use crate::{board::Board, error::SuDoKuError};
use superslice::*;

#[derive(Copy, Clone, Debug)]
pub struct Slot(usize, usize, usize);

/// solve a sudoku
/// the dfs way
#[derive(Default, Clone)]
pub struct DfsSolver {}

impl DfsSolver {
    pub fn new() -> DfsSolver {
        DfsSolver {}
    }

    /// find a solve of sudoku in dfs way
    pub fn solve(&self, board: &mut Board) -> Result<Vec<Slot>, SuDoKuError> {
        let mut queue = Vec::with_capacity(81);
        for x in 0..9 {
            for y in 0..9 {
                if board.is_empty(x, y) {
                    queue.push(Slot(x, y, board.avaliable_count(x, y)));
                }
            }
        }
        queue.sort_unstable_by(|a, b| a.2.cmp(&b.2));
        let mut cur = 0;
        while let Some(Slot(xr, yr, _)) = queue.get(cur) {
            let x = *xr;
            let y = *yr;
            let avaliables = board.avaliable_val(x, y);
            if avaliables.is_empty() {
                if cur != 0 {
                    cur -= 1;
                    board.unset(x, y);
                    continue;
                } else {
                    return Err(SuDoKuError::NotSolveable);
                }
            }
            let now = board.unchecked_get(x, y).unwrap_or(0);
            let mut nxt = 0;
            let upper_pos = avaliables.upper_bound(&now);
            if let Some(upper_than_now) = avaliables.get(upper_pos) {
                nxt = *upper_than_now;
            }
            if nxt == 0 {
                // no avaliable slot
                if cur != 0 {
                    cur -= 1;
                    board.unset(x, y);
                } else {
                    return Err(SuDoKuError::NotSolveable);
                }
            } else {
                cur += 1;
                board.unchecked_set(x, y, nxt);
            }
        }
        Ok(queue)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn solve() {
        let mut board = Board::from_vec(vec![
            0, 0, 0, 2, 0, 8, 7, 0, 9, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 0, 8,
            7, 0, 0, 4, 3, 0, 5, 6, 0, 0, 0, 5, 9, 0, 0, 1, 1, 9, 0, 3, 0, 2, 0, 0, 0, 9, 0, 8, 5,
            2, 6, 1, 0, 3, 5, 1, 6, 4, 3, 7, 9, 2, 8, 4, 2, 0, 8, 0, 0, 6, 5, 7,
        ]);
        let solver = DfsSolver::new();
        assert!(solver.solve(&mut board).is_ok());
        assert_eq!(
            "3 5 1 2 4 8 7 6 9
7 4 9 1 6 5 8 3 2
8 6 2 9 7 3 5 1 4
2 8 7 6 1 4 3 9 5
6 3 4 7 5 9 2 8 1
1 9 5 3 8 2 4 7 6
9 7 8 5 2 6 1 4 3
5 1 6 4 3 7 9 2 8
4 2 3 8 9 1 6 5 7\n"
                .to_string(),
            format!("{}", board)
        );
    }
}
