use crate::error::*;

#[derive(Default)]
pub struct Board {
    inner: Vec<u8>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            inner: vec![0; 81]
        }
    }

    pub fn unchecked_set(&mut self, x: usize, y: usize, val: u8) {
        let pos = x*9 + y;
        self.inner[pos] = val;
    }

    pub fn unchecked_get(&self, x: usize, y: usize) -> Option<u8> {
        let pos = x*9 + y;
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
