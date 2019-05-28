use crate::error::*;

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

    pub fn and(&mut self, another: Self) {
        self.inner |= another.inner;
    }

    pub fn count(self) -> usize {
        self.inner.count_ones() as usize
    }

    pub fn remove(&mut self, key: u8) -> Result<bool, SuDoKuError> {
        let before = self.get(key)?;
        self.inner &= !(1 << key);
        Ok(before)
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
        bitset_one.and(bitset_another);
        assert!(bitset_one.get(0).unwrap());
        assert!(bitset_one.get(1).unwrap());
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
}
