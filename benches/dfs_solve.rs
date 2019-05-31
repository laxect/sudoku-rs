#[macro_use]
extern crate criterion;
extern crate sudoku;

use criterion::{black_box, Criterion};

fn standard_sudoku(input: Vec<u8>) {
    let mut board = sudoku::board::Board::from_vec(input);
    let solver = sudoku::solver::DfsSolver::new();
    assert!(solver.solve(&mut board).is_ok());
}

fn standard_sudoku_bench(c: &mut Criterion) {
    c.bench_function("dfs solver", |b| {
        b.iter(|| {
            standard_sudoku(black_box(vec![
                0, 0, 0, 2, 0, 8, 7, 0, 9, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 0,
                8, 7, 0, 0, 4, 3, 0, 5, 6, 0, 0, 0, 5, 9, 0, 0, 1, 1, 9, 0, 3, 0, 2, 0, 0, 0, 9, 0,
                8, 5, 2, 6, 1, 0, 3, 5, 1, 6, 4, 3, 7, 9, 2, 8, 4, 2, 0, 8, 0, 0, 6, 5, 7,
            ]))
        })
    });
}

criterion_group!(benches, standard_sudoku_bench);
criterion_main!(benches);
