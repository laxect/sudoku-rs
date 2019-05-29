use crate::error::*;
use std::ops::BitOr;

#[derive(Default, Copy, Clone)]
pub struct BitSet {
    inner: u16,
}

impl BitSet {
    pub fn new() -> Self {
        BitSet { inner: 0 }
    }

    pub fn set(&mut self, key: u8) -> Result<(), SuDoKuError> {
        if key >= 16 {
            return Err(SuDoKuError::OutOfBound);
        }
        self.inner |= 1 << key;
        Ok(())
    }

    pub fn get(self, key: u8) -> Result<bool, SuDoKuError> {
        if key >= 16 {
            return Err(SuDoKuError::OutOfBound);
        }
        Ok(self.inner & (1 << key) != 0)
    }

    pub fn count(self) -> usize {
        self.inner.count_ones() as usize
    }

    pub fn remove(&mut self, key: u8) -> Result<bool, SuDoKuError> {
        let before = self.get(key)?;
        self.inner &= !(1 << key);
        Ok(before)
    }

    pub fn reverse(self, range: u8) -> Vec<u8> {
        let mut res = Vec::new();
        for i in 0..range {
            if !self.get(i).unwrap_or(true) {
                res.push(i);
            }
        }
        res
    }
}

impl BitOr for BitSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            inner: self.inner | rhs.inner
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn set() {
        let mut bitset = BitSet::new();
        bitset.set(1).unwrap();
        assert_eq!(bitset.inner, 2);
    }
    #[test]
    fn set_get() {
        let mut bitset = BitSet::new();
        bitset.set(2).unwrap();
        assert!(bitset.get(2).unwrap());
    }
    #[test]
    fn and() {
        let mut bitset_one = BitSet::new();
        bitset_one.set(1).unwrap();
        let mut bitset_another = BitSet::new();
        bitset_another.set(0).unwrap();
        let bitset_and = bitset_one | bitset_another;
        assert!(bitset_and.get(0).unwrap());
        assert!(bitset_and.get(1).unwrap());
    }
    #[test]
    fn count() {
        let mut bitset = BitSet::new();
        bitset.set(0).unwrap();
        bitset.set(1).unwrap();
        bitset.set(2).unwrap();
        bitset.set(1).unwrap();
        assert_eq!(bitset.inner, 7);
        assert_eq!(bitset.count(), 3);
    }
    #[test]
    fn remove() {
        let mut bitset = BitSet::new();
        bitset.set(1).unwrap();
        bitset.set(2).unwrap();
        bitset.remove(1).unwrap();
        assert_eq!(bitset.inner, 4);
    }
    #[test]
    fn reverse() {
        let mut bitset = BitSet::new();
        bitset.set(1).unwrap();
        bitset.set(2).unwrap();
        let v = bitset.reverse(4);
        assert_eq!(v, vec![0, 3]);
    }
}
