use crate::error::*;

#[derive(Default)]
pub struct BitSet {
    inner: u16,
}

impl BitSet {
    pub fn new() -> Self {
        BitSet { inner: 0 }
    }

    pub fn set(&mut self, key: usize) -> Result<(), SuDoKuError> {
        if key >= 16 {
            return Err(SuDoKuError::OutOfBound);
        }
        self.inner &= 1 << key;
        Ok(())
    }

    pub fn get(&self, key: usize) -> Result<bool, SuDoKuError> {
        if key >= 16 {
            return Err(SuDoKuError::OutOfBound);
        }
        Ok(self.inner & (1 << key) != 0)
    }

    pub fn and(&mut self, another: &Self) {
        self.inner &= another.inner;
    }

    pub fn count(&self) -> usize {
        self.inner.count_ones() as usize
    }
}
