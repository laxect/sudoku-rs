use crate::{board::Board, error::SuDoKuError};

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

    pub fn solve(&self, board: &Board) -> Result<Vec<Action>, SuDoKuError> {
        let mut stack = Vec::new();
        Ok(stack)
    }
}
