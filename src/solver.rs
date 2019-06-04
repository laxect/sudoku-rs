use crate::{board::Board, error::SuDoKuError};

type Slot = (usize, usize, usize);

/// solve a sudoku
/// in the dfs way
/// ```
/// use sudoku_rs::{board, solver};
///
/// let mut b = board::Board::new();
/// let s = solver::DfsSolver::new();
/// s.solve(&mut b);
/// println!("{}", b);
/// ```
#[derive(Default, Clone, Copy)]
pub struct DfsSolver {}

impl DfsSolver {
    pub fn new() -> DfsSolver {
        DfsSolver {}
    }

    /// check if the solve of a sudoku is unique
    /// ```
    /// use sudoku_rs::{board::Board, solver::DfsSolver};
    ///
    /// let mut board = Board::new();
    /// let solver = DfsSolver::new();
    /// assert_eq!(solver.unique(&mut board).unwrap(), false);
    /// ```
    pub fn unique(self, board: &mut Board) -> Result<bool, SuDoKuError> {
        let path = Vec::with_capacity(81);
        let path = self.solve_do(board, path)?;
        Ok(self.solve_do(board, path).is_err())
    }

    /// find a solve of sudoku in dfs way
    pub fn solve(self, board: &mut Board) -> Result<Vec<Slot>, SuDoKuError> {
        let path = Vec::with_capacity(81);
        self.solve_do(board, path)
    }

    fn solve_do(self, board: &mut Board, mut queue: Vec<Slot>) -> Result<Vec<Slot>, SuDoKuError> {
        let mut cur = if queue.is_empty() {
            for x in 0..9 {
                for y in 0..9 {
                    if board.is_empty(x, y) {
                        // (x, y, avaliable_count)
                        queue.push((x, y, board.avaliable_count(x, y)));
                    }
                }
            }
            // sort by avaliable count
            queue.sort_unstable_by(|a, b| a.2.cmp(&b.2));
            // avaliable count no use now
            queue.iter_mut().for_each(|item| item.2 = 0);
            0
        } else {
            queue.len() - 1
        };
        // (x, y, last_ind)
        while let Some((xr, yr, ind)) = queue.get_mut(cur) {
            let x = *xr;
            let y = *yr;
            if let Some(upper_than_now) = board.avaliable_val(x, y).get(*ind) {
                cur += 1;
                *ind += 1;
                board.unchecked_set(x, y, *upper_than_now);
            } else if cur != 0 {
                // no avaliable value
                cur -= 1;
                board.unset(x, y);
                *ind = 0;
            } else {
                // no avaliable slot
                return Err(SuDoKuError::NotSolveable);
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
    #[test]
    fn multi_solve() {
        let mut board = Board::new();
        let solver = DfsSolver::new();
        assert_eq!(solver.unique(&mut board).unwrap(), false);
    }
    #[test]
    fn unique() {
        let mut board = Board::from_vec(vec![
            0, 0, 0, 2, 0, 8, 7, 0, 9, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 0, 8,
            7, 0, 0, 4, 3, 0, 5, 6, 0, 0, 0, 5, 9, 0, 0, 1, 1, 9, 0, 3, 0, 2, 0, 0, 0, 9, 0, 8, 5,
            2, 6, 1, 0, 3, 5, 1, 6, 4, 3, 7, 9, 2, 8, 4, 2, 0, 8, 0, 0, 6, 5, 7,
        ]);
        let solver = DfsSolver::new();
        assert_eq!(solver.unique(&mut board).unwrap(), true);
    }
    #[test]
    fn unique_idempotence() {
        let board = Board::from_vec(vec![
            0, 0, 0, 2, 0, 8, 7, 0, 9, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 0, 8,
            7, 0, 0, 4, 3, 0, 5, 6, 0, 0, 0, 5, 9, 0, 0, 1, 1, 9, 0, 3, 0, 2, 0, 0, 0, 9, 0, 8, 5,
            2, 6, 1, 0, 3, 5, 1, 6, 4, 3, 7, 9, 2, 8, 4, 2, 0, 8, 0, 0, 6, 5, 7,
        ]);
        let solver = DfsSolver::new();
        assert_eq!(solver.unique(&mut board.clone()).unwrap(), true);
        assert_eq!(solver.unique(&mut board.clone()).unwrap(), true);
    }
}
