use crate::{bitset::BitSet, error::*};
use std::fmt;

#[derive(Default)]
pub struct Board {
    inner: Vec<u8>,
    mat: Vec<BitSet>,
    x: Vec<BitSet>,
    y: Vec<BitSet>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            inner: vec![0; 81],
            x: vec![BitSet::new(); 9],
            y: vec![BitSet::new(); 9],
            mat: vec![BitSet::new(); 9],
        }
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        let mut board = Board::new();
        for x in 0..9 {
            for y in 0..9 {
                let pos = x * 9 + y;
                board.unchecked_set(x, y, vec[pos]);
            }
        }
        board
    }

    pub fn unchecked_set(&mut self, x: usize, y: usize, val: u8) {
        let pos = x * 9 + y;
        let mat_id = (x / 3 * 3) + (y / 3);
        if self.inner[pos] != 0 {
            let before = self.inner[pos];
            self.x[x].remove(before).expect("x: should be a value");
            self.y[y].remove(before).expect("y: should be a value");
            self.mat[mat_id].remove(before).expect("should be a value");
        }
        self.inner[pos] = val;
        self.x[x].set(val).expect("x: out of bound");
        self.y[y].set(val).expect("y: out of bound");
        self.mat[mat_id].set(val).expect("mat: out of bound");
    }

    pub fn unchecked_get(&self, x: usize, y: usize) -> Option<u8> {
        let pos = x * 9 + y;
        let val = self.inner[pos];
        if val > 0 && val < 10 {
            Some(val)
        } else {
            None
        }
    }

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

    pub fn get(&self, x: usize, y: usize) -> Result<Option<u8>, SuDoKuError> {
        if x >= 9 && y >= 9 {
            return Err(SuDoKuError::OutOfBound);
        }
        Ok(self.unchecked_get(x, y))
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.get(x, y).unwrap_or(Some(0)).is_none()
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        let pos = x * 9 + y;
        let mat_id = (x / 3 * 3) + (y / 3);
        let before = self.inner[pos];
        if before != 0 {
            self.x[x].remove(before).expect("x: should be a value");
            self.y[y].remove(before).expect("y: should be a value");
            self.mat[mat_id]
                .remove(before)
                .expect("mat: should be a value");
            self.inner[pos] = 0;
        }
    }

    pub fn avaliable_val(&self, x: usize, y: usize) -> Vec<u8> {
        let pos = x * 9 + y;
        // just a backup
        // should not use this method when slot filled
        if self.inner[pos] != 0 {
            return Vec::new();
        }
        let mat_id = (x / 3 * 3) + (y / 3);
        let this = self.x[x] | self.y[y] | self.mat[mat_id];
        // return 0..9
        // but we need 1..=9
        this.reverse(10).into_iter().filter(|x| *x != 0).collect()
    }

    pub fn is_win(&self) -> bool {
        self.x.iter().filter(|bs| bs.count() == 9).count() == 9
            && self.y.iter().filter(|bs| bs.count() == 9).count() == 9
            && self.mat.iter().filter(|bs| bs.count() == 9).count() == 9
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
        board.set(1, 2, 3);
        assert!(!board.is_empty(1, 2));
        assert!(board.is_empty(2, 3));
    }
    #[test]
    fn avaliable_val() {
        let mut board = Board::new();
        board.set(1, 2, 3).unwrap();
        board.set(2, 3, 4).unwrap();
        assert_eq!(board.avaliable_val(1, 3), vec![1, 2, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn display() {
        let mut board = Board::new();
        board.set(1, 2, 3);
        assert_eq!("_ _ _ _ _ _ _ _ _\n_ _ 3 _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n".to_string(), format!("{}", board));
    }
}
