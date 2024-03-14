use std::time::Instant;

// The board dimens5ons.
const NUM_ROWS: usize = 6;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Return true if this series of squares contains at most one queen.
fn series_is_legal(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r0: i32,
    c0: i32,
    dr: i32,
    dc: i32,
) -> bool {
    let mut has_queen = false;

    let mut r = r0;
    let mut c = c0;
    loop {
        if board[r as usize][c as usize] == 'Q' {
            // If we already have a queen on this row,
            // then this board is not legal.
            if has_queen {
                return false;
            }

            // Remember that we have a queen on this row.
            has_queen = true;
        }

        // Move to the next square in the series.
        r += dr;
        c += dc;

        // If we fall off the board, then the series is legal.
        if r >= INUM_ROWS || c >= INUM_COLS || r < 0 || c < 0 {
            return true;
        }
    }
}

// Return true if the board is legal.
fn board_is_legal(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    // See if each row is legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, 0, 0, 1) {
            return false;
        }
    }

    // See if each column is legal.
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, 0) {
            return false;
        }
    }

    // See if diagonals down to the right are legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, 0, 1, 1) {
            return false;
        }
    }
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, 1) {
            return false;
        }
    }

    // See if diagonals down to the left are legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, INUM_ROWS - 1, 1, -1) {
            return false;
        }
    }
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, -1) {
            return false;
        }
    }

    // If we survived this long, then the board is legal.
    return true;
}

// Return true if the board is legal and a solution.
fn board_is_a_solution(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    // See if it is legal.
    if !board_is_legal(board) {
        return false;
    }

    // See if the board contains exactly NUM_ROWS queens.
    let mut num_queens = 0;
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            if board[r as usize][c as usize] == 'Q' {
                num_queens += 1;
            }
        }
    }
    return num_queens == NUM_ROWS;
}

fn place_queens_1(board: &mut [[char; NUM_COLS]; NUM_ROWS], r: i32, c: i32) -> bool {
    if r >= INUM_ROWS {
        return board_is_a_solution(board);
    }
    // Find the next square.
    let mut next_r = r;
    let mut next_c = c + 1;
    if next_c >= INUM_ROWS {
        next_r += 1;
        next_c = 0;
    }
    place_queens_1(board, next_r, next_c);
    if board_is_a_solution(board) {
        return true;
    }
    board[r as usize][c as usize] = 'Q';
    place_queens_1(board, next_r, next_c);
    if board_is_a_solution(board) {
        return true;
    }
    board[r as usize][c as usize] = '.';
    return false;
}

pub fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    let success = place_queens_1(&mut board, 0, 0);
    //let success = place_queens_2(& mut board, 0, 0, 0);
    //let success = place_queens_3(& mut board);
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
fn dump_board(board: &mut [[char; NUM_COLS]; NUM_ROWS]) {
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            print!("{:<02}", board[r][c]);
        }
        println!();
    }
    println!();
}
