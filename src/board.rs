//! the sudoku board mod
//!
//! ```
//! use sudoku_rs::board::Board;
//!
//! let sudoku_str = "400000805030000000000700000020000060000080400000010000000603070500200000104000000";
//! let board: Board = sudoku_str.parse().unwrap();
//! ```

use crate::{bitset::BitSet, error::*};
use std::{fmt, num::NonZeroU8};

type Grid = Option<NonZeroU8>;

/// board struct
#[derive(Clone)]
pub struct Board {
    inner: [Grid; 81],
    mat: [BitSet; 9],
    x: [BitSet; 9],
    y: [BitSet; 9],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            inner: [None; 81],
            x: [BitSet::new(); 9],
            y: [BitSet::new(); 9],
            mat: [BitSet::new(); 9],
        }
    }

    /// gen board from vec
    /// the len of vec must be 81
    /// 0 for empty and 1..=9 for value
    /// ```
    /// use sudoku_rs::board::Board;
    ///
    /// let b = Board::from_vec(vec![
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 0, 0, 0,
    /// ]);
    /// ```
    pub fn from_vec(mut vec: Vec<u8>) -> Self {
        if vec.len() < 81 {
            vec.append(&mut vec![0; 81 - vec.len()]);
        }
        let mut board = Board::new();
        for x in 0..9 {
            for y in 0..9 {
                let pos = x * 9 + y;
                let val = vec[pos];
                if val != 0 {
                    board.unchecked_set(x, y, vec[pos]);
                }
            }
        }
        board
    }

    /// set value in board but not check value
    /// will also set bitset
    pub fn unchecked_set(&mut self, x: usize, y: usize, val: u8) {
        let pos = x * 9 + y;
        let mat_id = (x / 3 * 3) + (y / 3);
        if let Some(before) = self.inner[pos].map(|nz| nz.get()) {
            self.x[x].remove(before).expect("x: should be a value");
            self.y[y].remove(before).expect("y: should be a value");
            self.mat[mat_id].remove(before).expect("should be a value");
        }
        self.inner[pos] = NonZeroU8::new(val);
        self.x[x].set(val).expect("x: out of bound");
        self.y[y].set(val).expect("y: out of bound");
        self.mat[mat_id].set(val).expect("mat: out of bound");
    }

    /// get value
    /// return a `Option<u8>`
    pub fn unchecked_get(&self, x: usize, y: usize) -> Option<u8> {
        let pos = x * 9 + y;
        let val = self.inner[pos];
        val.map(|nz| nz.get())
    }

    /// set value
    pub fn set(&mut self, x: usize, y: usize, val: u8) -> Result<(), SuDoKuError> {
        if x >= 9 && y >= 9 {
            return Err(SuDoKuError::OutOfBound);
        }
        if val > 9 || val < 1 {
            return Err(SuDoKuError::InvalidValue);
        }
        self.unchecked_set(x, y, val);
        Ok(())
    }

    /// get value
    pub fn get(&self, x: usize, y: usize) -> Result<Option<u8>, SuDoKuError> {
        if x >= 9 && y >= 9 {
            return Err(SuDoKuError::OutOfBound);
        }
        Ok(self.unchecked_get(x, y))
    }

    /// check a solt is empty of not
    /// ```
    /// use sudoku_rs::board::Board;
    ///
    /// let mut b = Board::new();
    /// b.set(1, 2, 3).unwrap();
    /// assert!(b.is_empty(1, 1));
    /// assert!(!b.is_empty(1, 2));
    /// ```
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.get(x, y).unwrap_or(Some(0)).is_none()
    }

    /// clear a slot
    pub fn unset(&mut self, x: usize, y: usize) {
        let pos = x * 9 + y;
        let mat_id = (x / 3 * 3) + (y / 3);
        if let Some(before) = self.inner[pos].map(|nz| nz.get()) {
            self.x[x].remove(before).expect("x: should be a value");
            self.y[y].remove(before).expect("y: should be a value");
            self.mat[mat_id]
                .remove(before)
                .expect("mat: should be a value");
            self.inner[pos] = None;
        }
    }

    /// get avaliable values for a slot
    pub fn avaliable_val(&self, x: usize, y: usize) -> Vec<u8> {
        let pos = x * 9 + y;
        let mat_id = (x / 3 * 3) + (y / 3);
        let mut cross = self.x[x] | self.y[y] | self.mat[mat_id];
        if let Some(this) = self.inner[pos] {
            cross.remove(this.get()).expect("range out");
        }
        cross.reverse(1..10)
    }

    /// get avaliable values count for a slot
    pub fn avaliable_count(&self, x: usize, y: usize) -> usize {
        let pos = x * 9 + y;
        let mat_id = (x / 3 * 3) + (y / 3);
        let mut cross = self.x[x] | self.y[y] | self.mat[mat_id];
        if let Some(this) = self.inner[pos] {
            cross.remove(this.get()).expect("range out");
        }
        9 - cross.count()
    }

    /// check if a board is filled
    pub fn is_win(&self) -> bool {
        self.x.iter().filter(|bs| bs.count() == 9).count() == 9
            && self.y.iter().filter(|bs| bs.count() == 9).count() == 9
            && self.mat.iter().filter(|bs| bs.count() == 9).count() == 9
    }
}

impl std::str::FromStr for Board {
    type Err = SuDoKuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        let mut chs = s.chars();
        for x in 0..9 {
            for y in 0..9 {
                let ch = chs.next().unwrap_or('.');
                if ch.is_ascii_digit() && ch != '0' {
                    board.set(x, y, ch.to_digit(10).unwrap() as u8)?;
                }
            }
        }
        Ok(board)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut matrix = String::new();
        for x in 0..9 {
            for y in 0..9 {
                let val = self.unchecked_get(x, y).unwrap_or(0);
                matrix.push(match val {
                    0 => '_',
                    n => (n + b'0').into(),
                });
                if y != 8 {
                    matrix.push(' ');
                }
            }
            matrix.push('\n');
        }
        write!(f, "{}", matrix)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn set_edge_0() {
        let mut board = Board::new();
        board.set(0, 0, 1).unwrap();
    }
    #[test]
    fn set_edge_81() {
        let mut board = Board::new();
        board.set(8, 8, 9).unwrap();
    }
    #[test]
    fn win_check() {
        let mut board = Board::from_vec(vec![
            9, 5, 3, 1, 2, 4, 7, 6, 8, 2, 4, 6, 3, 7, 8, 1, 5, 9, 7, 8, 1, 6, 5, 9, 2, 3, 4, 8, 6,
            9, 7, 1, 3, 5, 4, 2, 3, 2, 4, 8, 6, 5, 9, 1, 7, 1, 7, 5, 9, 4, 2, 6, 8, 3, 5, 9, 8, 2,
            3, 6, 4, 7, 1, 4, 1, 2, 5, 8, 7, 3, 9, 6, 6, 3, 7, 4, 9, 1, 8, 2, 5,
        ]);
        assert!(board.is_win());
        board.unchecked_set(0, 0, 1);
        assert!(!board.is_win());
    }
    #[test]
    fn empty() {
        let mut board = Board::new();
        board.set(1, 2, 3).unwrap();
        assert!(!board.is_empty(1, 2));
        assert!(board.is_empty(2, 3));
    }
    #[test]
    fn avaliable_count() {
        let mut board = Board::new();
        board.set(1, 2, 3).unwrap();
        board.set(2, 3, 4).unwrap();
        board.set(1, 3, 7).unwrap();
        assert_eq!(board.avaliable_count(1, 3), 7);
    }
    #[test]
    fn avaliable_val() {
        let mut board = Board::new();
        board.set(1, 2, 3).unwrap();
        board.set(2, 3, 4).unwrap();
        board.set(1, 3, 7).unwrap();
        assert_eq!(board.avaliable_val(1, 3), vec![1, 2, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn display() {
        let mut board = Board::new();
        board.set(1, 2, 3).unwrap();
        assert_eq!("_ _ _ _ _ _ _ _ _\n_ _ 3 _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n".to_string(), format!("{}", board));
    }
    #[test]
    fn from_str() {
        let sudoku =
            "400000805030000000000700000020000060000080400000010000000603070500200000104000000";
        let board: Board = sudoku.parse().unwrap();
        assert_eq!(board.get(6, 3).unwrap().unwrap_or(0), 6);
    }
}
