use rustsudoku::*;

fn main() {
    let mut b = SudokuBoard::new([
        [1, 0, 2, 0, 0, 0, 0, 0, 0],
        [0, 4, 9, 0, 0, 0, 0, 0, 0],
        [0, 8, 6, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    dbg!(b.check_board());
    b.print_board();

    let mut solver = Solver::new(&mut b);
    solver.solve(BoardPos::new(0, 0));

    b.print_board();
}
