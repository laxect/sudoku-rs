use crate::{board::Board, error::SuDoKuError};
use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub struct Action(usize, usize);

/// solve a sudoku
/// the dfs way
#[derive(Default, Clone)]
pub struct DfsSolver {}

impl DfsSolver {
    pub fn new() -> DfsSolver {
        DfsSolver {}
    }

    // just find one solve
    pub fn solve(&self, board: &mut Board) -> Result<Vec<Action>, SuDoKuError> {
        let mut stack = Vec::with_capacity(81);
        let mut next = VecDeque::with_capacity(81);
        for x in 0..9 {
            for y in 0..9 {
                if board.is_empty(x, y) {
                    next.push_back(Action(x, y));
                }
            }
        }
        while let Some(Action(x, y)) = next.pop_front() {
            let avaliables = board.avaliable_val(x, y);
            if avaliables.is_empty() {
                next.push_front(Action(x, y));
                if let Some(end) = stack.pop() {
                    next.push_front(end);
                    continue;
                } else {
                    return Err(SuDoKuError::NotSolveable);
                }
            }
            let now = board.unchecked_get(x, y).unwrap_or(0);
            let mut nxt = 0;
            for i in avaliables.into_iter() {
                if i > now {
                    nxt = i;
                    break;
                }
            }
            if nxt == 0 {
                // no avaliable slot
                next.push_front(Action(x, y));
                if let Some(end) = stack.pop() {
                    next.push_front(end);
                } else {
                    return Err(SuDoKuError::NotSolveable);
                }
            } else {
                stack.push(Action(x, y));
                board.unchecked_set(x, y, nxt);
            }
        }
        Ok(stack)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn solve() {
        let mut board = Board::from_vec(vec![
            0, 0, 0, 2, 0, 8, 7, 0, 9,
            0, 4, 0, 1, 0, 0, 0, 0, 0,
            0, 6, 0, 0, 0, 3, 0, 0, 0,
            0, 8, 7, 0, 0, 4, 3, 0, 5,
            6, 0, 0, 0, 5, 9, 0, 0, 1,
            1, 9, 0, 3, 0, 2, 0, 0, 0,
            9, 0, 8, 5, 2, 6, 1, 0, 3,
            5, 1, 6, 4, 3, 7, 9, 2, 8,
            4, 2, 0, 8, 0, 0, 6, 5, 7,
        ]);
        let solver = DfsSolver::new();
        assert!(!solver.solve(&mut board).is_ok());
        assert_eq!("".to_string(), format!("{}", board));
    }
}
