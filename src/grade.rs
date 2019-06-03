use crate::board;

/// instand of grand return
/// dfs complexity show us how hard to find a correct solution
/// and empty slot count show how many slot need to fill
#[derive(Copy, Clone, Debug)]
pub struct Score {
    pub dfs_complexity: usize,
    pub empty_slot_count: usize,
}

/// use human-like dfs algorithm for sudoku grade
/// ```
/// use sudoku_rs::{board, grade};
/// 
/// let mut b = board::Board::new();
/// let g = grade::Grade::new();
/// let scores = g.grade(&mut b);
/// println!("{:?}", scores);
/// ```
/// this mod doesn't care of unique
/// but inunique sudoku's complexity will be higher on average
#[derive(Default, Copy, Clone)]
pub struct Grade {}

impl Grade {
    /// return a new Grade struct
    pub fn new () -> Self {
        Grade {}
    }

    /// grade a sudoku
    pub fn grade(self, target: &mut board::Board) -> Score {
        let mut queue = Vec::with_capacity(81);
        for x in 0..9 {
            for y in 0..9 {
                if target.is_empty(x, y) {
                    // (x, y, avaliable_count)
                    queue.push((x, y, target.avaliable_count(x, y)));
                }
            }
        }
        // avaliable count no use now
        queue.sort_unstable_by(|a, b| a.2.cmp(&b.2));
        queue.iter_mut().for_each(|item| item.2 = 0);
        // do the dfs
        let mut cur = 0;
        let mut dfs_complexity = 0;
        // only finish when back to start point
        while let Some((rx, ry, ind)) = queue.get_mut(cur) {
            let x = *rx;
            let y = *ry;
            if let Some(upper_than_now) = target.avaliable_val(x, y).get(*ind) {
                cur += 1;
                *ind += 1;
                target.unchecked_set(x, y, *upper_than_now);
            } else {
                // no avaliable value
                dfs_complexity += 1;
                if cur != 0 {
                    cur -= 1;
                    target.unset(x, y);
                    *ind = 0;
                } else {
                    // no avaliable slot
                    break;
                }
            }
            // no finsih even when sudoku complete
            if cur == queue.len() {
                if dfs_complexity > 100_000 {
                    // doesn't count more than 100_000
                    break;
                }
                cur = cur.checked_sub(2).unwrap_or(0);
                target.unset(x, y);
            }
        }
        Score {
            dfs_complexity,
            empty_slot_count: queue.len(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn grade_0() {
        let mut b = board::Board::from_vec(vec![
            9, 5, 3, 1, 2, 4, 7, 6, 8, 2, 4, 6, 3, 7, 8, 1, 5, 9, 7, 8, 1, 6, 5, 9, 2, 3, 4, 8, 6,
            9, 7, 1, 3, 5, 4, 2, 3, 2, 4, 8, 6, 5, 9, 1, 7, 1, 7, 0, 9, 4, 2, 6, 8, 3, 5, 9, 8, 2,
            3, 6, 4, 7, 1, 4, 1, 2, 5, 8, 7, 3, 9, 6, 6, 3, 7, 4, 9, 1, 8, 2, 5,
        ]);
        let g = Grade::new();
        let score = g.grade(&mut b);
        assert_eq!(score.dfs_complexity, 1);
    }
    #[test]
    fn grade_9() {
        let mut b = board::Board::from_vec(vec![
            0, 0, 3, 1, 2, 4, 7, 6, 8, 2, 4, 6, 3, 7, 8, 1, 5, 9, 7, 8, 1, 6, 5, 9, 2, 3, 4, 8, 6,
            9, 7, 1, 3, 5, 4, 2, 3, 2, 4, 8, 6, 5, 9, 1, 7, 1, 7, 0, 9, 4, 2, 6, 8, 3, 5, 9, 8, 2,
            3, 6, 4, 7, 1, 4, 1, 2, 5, 8, 7, 3, 9, 6, 6, 3, 7, 4, 9, 1, 8, 2, 5,
        ]);
        let g = Grade::new();
        let score = g.grade(&mut b);
        assert_eq!(score.dfs_complexity, 2);
    }
    #[test]
    fn grade_n() {
        let mut b = board::Board::from_vec(vec![
            0, 0, 0, 2, 0, 8, 7, 0, 9, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 0, 8,
            7, 0, 0, 4, 3, 0, 5, 6, 0, 0, 0, 5, 9, 0, 0, 1, 1, 9, 0, 3, 0, 2, 0, 0, 0, 9, 0, 8, 5,
            2, 6, 1, 0, 3, 5, 1, 6, 4, 3, 7, 9, 2, 8, 4, 2, 0, 8, 0, 0, 6, 5, 7,
        ]);
        let g = Grade::new();
        let score = g.grade(&mut b);
        assert_ne!(score.dfs_complexity, 0);
    }
}