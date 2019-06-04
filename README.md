# sudoku-rs
sudoku solve lib

<a href="https://travis-ci.org/laxect/sudoku-rs"><img src="https://travis-ci.org/laxect/sudoku-rs.svg?branch=master"></a>
<a href="https://crates.io/crates/sudoku_rs"><img src="https://img.shields.io/crates/v/sudoku_rs.svg"></a>
<a href="https://docs.rs/sudoku_rs/"><img src="https://docs.rs/sudoku_rs/badge.svg"></a>

sudoku solve
```
use sudoku_rs::{board, solver};

let mut b = board::Board::new();
let mut s = solver::DfsSolver::new();
s.solve(&mut b);
println!("{}", b);
```

unique
```
use sudoku_rs::{board, solver};

let mut b = board::Board::new();
let mut s = solver::DfsSolver::new();
assert!(!s.unique(&mut b).unwrap());
```

grade
```
use sudoku_rs::{board, grade};
//!
let mut b = board::Board::new();
let g = grade::Grade::new();
let scores = g.grade(&mut b);
println!("{:?}", scores);
```


next to do

 - [x] dfs solve
 - [x] unique check
 - [x] sudoku grade
 - [ ] sudoku gen
