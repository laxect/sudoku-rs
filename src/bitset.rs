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
        self.inner |= 1 << key;
        Ok(())
    }

    pub fn get(&self, key: usize) -> Result<bool, SuDoKuError> {
        if key >= 16 {
            return Err(SuDoKuError::OutOfBound);
        }
        Ok(self.inner & (1 << key) != 0)
    }

    pub fn and(&mut self, another: &Self) {
        self.inner |= another.inner;
    }

    pub fn count(&self) -> usize {
        self.inner.count_ones() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn set() {
        let mut bitset = BitSet::new();
        bitset.set(1);
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
        bitset_one.set(1);
        let mut bitset_another = BitSet::new();
        bitset_another.set(0);
        bitset_one.and(&bitset_another);
        assert!(bitset_one.get(0).unwrap());
        assert!(bitset_one.get(1).unwrap());
    }
    #[test]
    fn count() {
        let mut bitset = BitSet::new();
        let keys = vec![0, 1, 2, 1];
        bitset.set(0);
        bitset.set(1);
        bitset.set(2);
        bitset.set(1);
        assert_eq!(bitset.inner, 7);
        assert_eq!(bitset.count(), 3);
    }
}