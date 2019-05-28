use crate::{bitset::BitSet, error::*};

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

    pub fn unchecked_set(&mut self, x: usize, y: usize, val: u8) {
        let pos = x * 9 + y;
        if self.inner[pos] != 0 {
        }
        self.inner[pos] = val;
        self.x[x].set(val).expect("x: out of bound");
        self.y[y].set(val).expect("y: out of bound");
        let mat_id = (x / 3 * 3) + (y / 3);
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

    pub fn get(&mut self, x: usize, y: usize) -> Result<Option<u8>, SuDoKuError> {
        if x >= 9 && y >= 9 {
            return Err(SuDoKuError::OutOfBound);
        }
        Ok(self.unchecked_get(x, y))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn set_edge_0() {
        let mut board = Board::new();
        board.set(0, 0, 1);
    }
    #[test]
    fn set_edge_81() {
        let mut board = Board::new();
        board.set(8, 8, 9);
    }
}
