#[derive(Debug)]
struct SudokuBoard {
    rows: [[u32; 9]; 9],
}

// takes two vectors of possibly separate types, then produces a vector which
// contains all the possible combinations of the elements.
fn combinator<T: Copy, U: Copy>(l1: Vec<T>, l2: Vec<U>) -> Vec<(T, U)> {
    l1.into_iter().flat_map(|x| l2.clone().into_iter().map(move |y| (x, y))).collect()
}

impl SudokuBoard {
    fn new(rows: [[u32; 9]; 9]) -> SudokuBoard {
        SudokuBoard { rows }
    }

    fn print_board(&self) {
        for row in self.rows {
            for digit in row {
                if digit == 0 {
                    // digit 0 regarded as empty spot
                    print!("_ ");
                } else {
                    print!("{} ", digit);
                }
            }
            println!();
        }
    }

    fn check_board(&self) -> bool {
        self.check_rows() && self.check_columns() && self.check_regions()
    }

    fn check_regions(&self) -> bool {
        let regions = combinator(vec![0, 1, 2], vec![0, 1, 2]);
        regions.into_iter().all(|(col_nbr, row_nbr)| self.check_region(col_nbr, row_nbr))
    }

    fn check_columns(&self) -> bool {
        (0..9).all(|col_nbr| self.check_column(col_nbr))
    }

    fn check_rows(&self) -> bool {
        (0..9).all(|row_nbr| self.check_row(row_nbr))
    }

    fn check_row(&self, row_nbr: usize) -> bool {
        let row = self.rows[row_nbr];

        let mut tracker = DigitTracker::new();

        for digit in row {
            tracker.found_digit(digit);
        }

        tracker.okay
    }

    fn check_column(&self, col_nbr: usize) -> bool {
        let mut col = Vec::new();

        for row in self.rows {
            col.push(row[col_nbr]);
        }

        let mut tracker = DigitTracker::new();

        for digit in col {
            tracker.found_digit(digit);
        }

        tracker.okay
    }

    fn check_region(&self, reg_x: usize, reg_y: usize) -> bool {
        let mut tracker = DigitTracker::new();

        for row_nbr in reg_y * 3..(reg_y + 1) * 3 {
            for col_nbr in reg_x * 3..(reg_x + 1) * 3 {
                let digit = self.rows[row_nbr][col_nbr];
                tracker.found_digit(digit);
            }
        }

        tracker.okay
    }

    fn set_digit(&mut self, position: BoardPos, digit: u32) {
        self.rows[position.row][position.col] = digit;
    }

    fn remove_digit(&mut self, position: BoardPos) {
        self.set_digit(position, 0);
    }

    fn get_digit(&mut self, position: BoardPos) -> u32 {
        self.rows[position.row][position.col]
    }
}

struct DigitTracker {
    found_digits: [bool; 9],
    okay: bool,
}

impl DigitTracker {
    fn new() -> DigitTracker {
        DigitTracker {
            found_digits: [false; 9],
            okay: true,
        }
    }

    fn found_digit(&mut self, digit: u32) {
        if digit != 0 {
            if self.found_digits[(digit - 1) as usize] {
                self.okay = false;
            } else {
                self.found_digits[(digit - 1) as usize] = true;
            }
        }
    }
}

#[derive(Copy, Clone)]
struct BoardPos {
    row: usize,
    col: usize,
}

impl BoardPos {
    fn new(row: usize, col: usize) -> BoardPos {
        if row > 8 || col > 8 {
            panic!("The board position {row}, {col} is not on the board!");
        }
        BoardPos { row, col }
    }

    fn is_at_end(&self) -> bool {
        self.col == 8 && self.row == 8
    }

    fn next_pos(&self) -> BoardPos {
        let new_col;
        let new_row;

        if self.is_at_end() {
            panic!("next_pos called at end of board");
        }

        if self.col == 8 {
            new_col = 0;
            new_row = self.row + 1;
        } else {
            new_col = self.col + 1;
            new_row = self.row;
        }

        BoardPos {
            row: new_row,
            col: new_col,
        }
    }
}

struct Solver<'a> {
    board: &'a mut SudokuBoard,
}

impl<'a> Solver<'a> {
    fn new(board: &'a mut SudokuBoard) -> Solver {
        Solver { board }
    }

    fn solve(&mut self, curr_pos: BoardPos) -> bool {
        let digit = self.board.get_digit(curr_pos);

        if digit == 0 {
            for digit_to_try in 1..=9 {
                self.board.set_digit(curr_pos, digit_to_try);

                if self.board.check_board() {
                    if !curr_pos.is_at_end() {
                        if self.solve(curr_pos.next_pos()) {
                            return true;
                        }
                    } else {
                        return true;
                    }
                }
            }
            self.board.remove_digit(curr_pos); // remove the digit we placed
            return false;
        }

        if curr_pos.is_at_end() {
            self.board.check_board()
        } else {
            self.solve(curr_pos.next_pos())
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_board_should_be_right() {
        let b = SudokuBoard::new([
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
        assert!(b.check_board());
    }

    #[test]
    fn check_column_not_right() {
        let b = SudokuBoard::new([
            [1, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 4, 9, 0, 0, 0, 0, 0, 0],
            [0, 8, 6, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        assert!(!b.check_board());
    }

    #[test]
    fn check_row_not_right() {
        let b = SudokuBoard::new([
            [1, 0, 2, 0, 1, 0, 0, 0, 0],
            [0, 4, 9, 0, 0, 0, 0, 0, 0],
            [0, 8, 6, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        assert!(!b.check_board());
    }

    #[test]
    fn check_region_not_right() {
        let b = SudokuBoard::new([
            [1, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 4, 9, 0, 0, 0, 0, 0, 0],
            [0, 8, 6, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 2, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        assert!(!b.check_board());
    }

    #[test]
    fn check_region_right() {
        let b = SudokuBoard::new([
            [1, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 4, 9, 0, 0, 0, 0, 0, 0],
            [0, 8, 6, 0, 0, 0, 0, 0, 0],
            [9, 1, 3, 0, 0, 0, 2, 0, 0],
            [2, 5, 4, 0, 0, 0, 0, 0, 0],
            [8, 6, 7, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        assert!(b.check_board());
    }

    #[test]
    fn check_complete_board_is_right() {
        let b = SudokuBoard::new([
            [1, 3, 2, 4, 5, 6, 7, 8, 9],
            [5, 4, 9, 1, 7, 8, 2, 3, 6],
            [7, 8, 6, 2, 3, 9, 4, 5, 1],
            [2, 1, 7, 9, 4, 3, 8, 6, 5],
            [6, 5, 8, 7, 2, 1, 9, 4, 3],
            [3, 9, 4, 6, 8, 5, 1, 2, 7],
            [9, 2, 1, 3, 6, 4, 5, 7, 8],
            [8, 7, 3, 5, 1, 2, 6, 9, 4],
            [4, 6, 5, 8, 9, 7, 3, 1, 2],
        ]);
        assert!(b.check_board());
    }

    #[test]
    fn solvable_board_is_solvable() {
        let mut b = SudokuBoard::new([
            [1, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 4, 9, 0, 0, 0, 0, 0, 0],
            [0, 8, 6, 0, 0, 0, 0, 0, 0],
            [9, 1, 3, 0, 0, 0, 2, 0, 0],
            [2, 5, 4, 0, 0, 0, 0, 0, 0],
            [8, 6, 7, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        let mut s = Solver::new(&mut b);

        assert!(s.solve(BoardPos::new(0, 0)));
    }

    #[test]
    fn unsolvable_board_is_not_solvable() {
        let mut b = SudokuBoard::new([
            [1, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 4, 9, 0, 0, 0, 0, 0, 0],
            [0, 8, 6, 0, 0, 0, 0, 0, 0],
            [9, 1, 3, 0, 0, 0, 2, 0, 0],
            [2, 4, 4, 0, 0, 0, 0, 0, 0],
            [8, 6, 7, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        let mut s = Solver::new(&mut b);

        assert!(!s.solve(BoardPos::new(0, 0)));
    }
}
