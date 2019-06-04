//! a lib for sudoku
//!
//! sudoku solve
//! ```
//! use sudoku_rs::{board, solver};
//!
//! let mut b = board::Board::new();
//! let s = solver::DfsSolver::new();
//! s.solve(&mut b);
//! println!("{}", b);
//! ```
//!
//! unique
//! ```
//! use sudoku_rs::{board, solver};
//!
//! let mut b = board::Board::new();
//! let s = solver::DfsSolver::new();
//! assert!(!s.unique(&mut b).unwrap());
//! ```
//!
//! grade
//! ```
//! use sudoku_rs::{board, grade};
//!
//! let mut b = board::Board::new();
//! let g = grade::Grade::new();
//! let scores = g.grade(&mut b);
//! println!("{:?}", scores);
//! ```

mod bitset;
pub mod board;
pub mod error;
pub mod grade;
pub mod solver;
