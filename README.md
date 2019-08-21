# sudoku-rs
sudoku solve lib

[![Build Status](https://dev.azure.com/GyaraStudio/sudoku/_apis/build/status/laxect.sudoku-rs?branchName=master)](https://dev.azure.com/GyaraStudio/sudoku/_build/latest?definitionId=2&branchName=master)
[![Actions Status](https://github.com/laxect/sudoku-rs/workflows/Test%20Rust%20project/badge.svg)](https://github.com/laxect/sudoku-rs/actions)
<a href="https://crates.io/crates/sudoku_rs"><img src="https://img.shields.io/crates/v/sudoku_rs.svg"></a>
<a href="https://docs.rs/sudoku_rs/"><img src="https://docs.rs/sudoku_rs/badge.svg"></a>

sudoku solve
```
use sudoku_rs::{board, solver};

let mut b = board::Board::new();
let s = solver::DfsSolver::new();
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
