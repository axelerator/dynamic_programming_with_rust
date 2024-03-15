use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 5;
const NUM_COLS: usize = NUM_ROWS;
const NUM_FIELDS: usize = NUM_ROWS * NUM_COLS;
const NUM_QUEENS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Empty,
    Queen,
}

struct Board([[Field; NUM_COLS]; NUM_ROWS]);

enum Direction {
    Horizontal,
    Vertical,
    DiagonalRight,
    DiagonalLeft,
}

#[derive(Clone)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    fn origin() -> Position {
        Position { row: 0, column: 0 }
    }

    fn move_in(&mut self, direction: &Direction) {
        match direction {
            Direction::Horizontal => self.column += 1,
            Direction::Vertical => self.row += 1,
            Direction::DiagonalRight => {
                self.row += 1;
                self.column += 1
            }
            Direction::DiagonalLeft => {
                self.row += 1;
                self.column -= 1
            }
        }
    }

    fn next(self: &Position) -> Position {
        let mut next_r = self.row;
        let mut next_c = self.column + 1;
        if next_c >= INUM_ROWS {
            next_r += 1;
            next_c = 0;
        }
        return Position {
            row: next_r,
            column: next_c,
        };
    }

    fn is_outside_board(self: &Position) -> bool {
        self.row >= INUM_ROWS || self.row < 0 || self.column < 0 || self.column >= INUM_COLS
    }
}

impl Board {
    fn empty() -> Board {
        Board([[Field::Empty; NUM_COLS]; NUM_ROWS])
    }

    fn holds_queen(self: &Board, pos: &Position) -> bool {
        self.0[pos.row as usize][pos.column as usize] == Field::Queen
    }
    //
    // Return true if this series of squares contains at most one queen.
    fn series_is_legal(self: &mut Board, pos: Position, direction: Direction) -> bool {
        let mut has_queen = false;

        let mut pos = pos.clone();
        loop {
            if self.holds_queen(&pos) {
                // If we already have a queen on this row,
                // then this board is not legal.
                if has_queen {
                    return false;
                }

                // Remember that we have a queen on this row.
                has_queen = true;
            }

            // Move to the next square in the series.
            pos.move_in(&direction);

            // If we fall off the board, then the series is legal.
            if pos.is_outside_board() {
                return true;
            }
        }
    }

    // Return true if the board is legal.
    fn board_is_legal(self: &mut Board) -> bool {
        // See if each row is legal.
        for r in 0..INUM_ROWS {
            if !self.series_is_legal(start_of_row(r), Direction::Horizontal) {
                return false;
            }
        }

        // See if each column is legal.
        for c in 0..INUM_COLS {
            if !self.series_is_legal(start_of_column(c), Direction::Vertical) {
                return false;
            }
        }

        // See if diagonals down to the right are legal.
        for r in 0..INUM_ROWS {
            if !self.series_is_legal(start_of_row(r), Direction::DiagonalRight) {
                return false;
            }
        }
        for c in 0..INUM_COLS {
            if !self.series_is_legal(start_of_column(c), Direction::DiagonalRight) {
                return false;
            }
        }

        // See if diagonals down to the left are legal.
        for r in 0..INUM_ROWS {
            if !self.series_is_legal(end_of_row(r), Direction::DiagonalLeft) {
                return false;
            }
        }
        for c in 0..INUM_COLS {
            if !self.series_is_legal(start_of_column(c), Direction::DiagonalLeft) {
                return false;
            }
        }

        // If we survived this long, then the board is legal.
        return true;
    }

    // Return true if the board is legal and a solution.
    fn board_is_a_solution(self: &mut Board) -> bool {
        // See if it is legal.
        if !self.board_is_legal() {
            return false;
        }

        // See if the board contains exactly NUM_ROWS queens.
        let mut num_queens = 0;
        let mut pos = Position { row: 0, column: 0 };
        for _ in 0..NUM_FIELDS {
            if self.holds_queen(&pos) {
                num_queens += 1;
            }
            pos = pos.next();
        }
        return num_queens == NUM_ROWS;
    }

    fn place_queens_1(self: &mut Board, pos: Position) -> bool {
        if pos.is_outside_board() {
            return self.board_is_a_solution();
        }
        // Find the next square.
        let next_pos = pos.next();

        self.place_queens_1(next_pos.clone());
        if self.board_is_a_solution() {
            return true;
        }
        self.put_queen_at(&pos);

        self.place_queens_1(next_pos);
        if self.board_is_a_solution() {
            return true;
        }
        self.remove_queen_from(&pos);
        return false;
    }

    fn put_queen_at(self: &mut Board, pos: &Position) {
        self.0[pos.row as usize][pos.column as usize] = Field::Queen;
    }

    fn remove_queen_from(self: &mut Board, pos: &Position) {
        self.0[pos.row as usize][pos.column as usize] = Field::Empty;
    }

    fn place_queens_2(self: &mut Board, pos: Position, num_placed: usize) -> bool {
        if num_placed >= NUM_QUEENS {
            return self.board_is_a_solution();
        }

        if pos.is_outside_board() {
            return self.board_is_a_solution();
        }

        // Find the next square.
        let next_pos = pos.next();
        self.place_queens_2(next_pos.clone(), num_placed);
        if self.board_is_a_solution() {
            return true;
        }

        self.put_queen_at(&pos);
        self.place_queens_2(next_pos, num_placed + 1);
        if self.board_is_a_solution() {
            return true;
        }

        self.remove_queen_from(&pos);
        return false;
    }

    // Set up and call place_queens_3.
    fn place_queens_3(self: &mut Board) -> bool {
        // Make the num_attacking array.
        // The value num_attacking[r as usize][c as usize] is the number
        // of queens that can attack square (r, c).
        let mut num_attacking = [[0; NUM_COLS]; NUM_ROWS];

        // Call do_place_queens_3.
        let num_placed = 0;
        return self.do_place_queens_3(num_placed, Position::origin(), &mut num_attacking);
    }

    // Try placing a queen at position [r][c].
    // Keep track of the number of queens placed.
    // Keep running totals of the number of queens attacking a square.
    // Return true if we find a legal board.
    fn do_place_queens_3(
        self: &mut Board,
        mut num_placed: usize,
        pos: Position,
        num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
    ) -> bool {
        // See if we have placed all of the queens.
        if num_placed == NUM_QUEENS {
            // See if this is a solution.
            return self.board_is_a_solution();
        }

        // See if we have examined the whole board.
        if pos.is_outside_board() {
            // We have examined all of the squares but this is not a solution.
            return false;
        }

        // Find the next square.
        let next_pos = pos.next();

        // Leave no queen in this square and
        // recursively assign the next square.
        if self.do_place_queens_3(num_placed, next_pos.clone(), num_attacking) {
            return true;
        }

        // See if we can place a queen at (r, c).
        if num_attacking[pos.row as usize][pos.column as usize] == 0 {
            // Try placing a queen here and
            // recursively assigning the next square.
            self.put_queen_at(&pos);
            num_placed += 1;

            // Increment the attack counts for this queen.
            adjust_attack_counts(num_attacking, &pos, 1);

            if self.do_place_queens_3(num_placed, next_pos, num_attacking) {
                return true;
            }

            // That didn't work so remove this queen.
            self.remove_queen_from(&pos);
            adjust_attack_counts(num_attacking, &pos, -1);
        }

        // If we get here, then there is no solution from
        // the board position before this function call.
        // Return false to backtrack and try again farther up the call stack.
        return false;
    }
}

// Add amount to the attack counts for this square.
fn adjust_attack_counts(
    num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
    pos: &Position,
    amount: i32,
) {
    // Attacks in the same row.
    for i in 0..INUM_COLS {
        num_attacking[pos.row as usize][i as usize] += amount
    }

    // Attacks in the same column.
    for i in 0..INUM_ROWS {
        num_attacking[i as usize][pos.column as usize] += amount
    }

    // Attacks in the upper left to lower right diagonal.
    for i in -INUM_ROWS..INUM_ROWS {
        let test_r = pos.row + i;
        let test_c = pos.column + i;
        if test_r >= 0 && test_r < INUM_ROWS && test_c >= 0 && test_c < INUM_ROWS {
            num_attacking[test_r as usize][test_c as usize] += amount;
        }
    }

    // Attacks in the upper right to lower left diagonal.
    for i in -INUM_ROWS..INUM_ROWS {
        let test_r = pos.row + i;
        let test_c = pos.column - i;
        if test_r >= 0 && test_r < INUM_ROWS && test_c >= 0 && test_c < INUM_ROWS {
            num_attacking[test_r as usize][test_c as usize] += amount;
        }
    }
}

pub fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = Board::empty();

    let start = Instant::now();
    let origin = Position::origin();
    let approach = 1;
    let success = match approach {
        1 => board.place_queens_1(origin),
        2 => board.place_queens_2(origin, 0),
        3 => board.place_queens_3(),
        _ => panic!("approach must be 1, 2, or 3"),
    };
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

// Display the board.
fn dump_board(board: &mut Board) {
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            let c = match board.0[r][c] {
                Field::Empty => ".",
                Field::Queen => "Q",
            };
            print!("{:<02}", c);
        }
        println!();
    }
    println!();
}

fn start_of_row(row: i32) -> Position {
    Position { row, column: 0 }
}

fn start_of_column(column: i32) -> Position {
    Position { row: 0, column }
}

fn end_of_row(row: i32) -> Position {
    Position {
        row,
        column: INUM_COLS - 1,
    }
}
