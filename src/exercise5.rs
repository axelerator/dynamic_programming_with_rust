use std::time::Instant;

// The board dimens5ons.
const NUM_ROWS: usize = 10;
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

fn place_queens_2(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r: i32,
    c: i32,
    num_placed: usize,
) -> bool {
    if num_placed >= NUM_ROWS {
        return board_is_a_solution(board);
    }

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
    place_queens_2(board, next_r, next_c, num_placed);
    if board_is_a_solution(board) {
        return true;
    }
    board[r as usize][c as usize] = 'Q';
    place_queens_2(board, next_r, next_c, num_placed + 1);
    if board_is_a_solution(board) {
        return true;
    }
    board[r as usize][c as usize] = '.';
    return false;
}

// Set up and call place_queens_3.
fn place_queens_3(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    // Make the num_attacking array.
    // The value num_attacking[r as usize][c as usize] is the number
    // of queens that can attack square (r, c).
    let mut num_attacking = [[0; NUM_COLS]; NUM_ROWS];

    // Call do_place_queens_3.
    let mut num_placed = 0;
    return do_place_queens_3(board, num_placed, 0, 0, &mut num_attacking);
}

// Try placing a queen at position [r][c].
// Keep track of the number of queens placed.
// Keep running totals of the number of queens attacking a square.
// Return true if we find a legal board.
fn do_place_queens_3(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    mut num_placed: i32,
    r: i32,
    c: i32,
    num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
) -> bool {
    // See if we have placed all of the queens.
    if num_placed == INUM_ROWS {
        // See if this is a solution.
        return board_is_a_solution(board);
    }

    // See if we have examined the whole board.
    if r >= INUM_ROWS {
        // We have examined all of the squares but this is not a solution.
        return false;
    }

    // Find the next square.
    let mut next_r = r;
    let mut next_c = c + 1;
    if next_c >= INUM_ROWS {
        next_r += 1;
        next_c = 0;
    }

    // Leave no queen in this square and
    // recursively assign the next square.
    if do_place_queens_3(board, num_placed, next_r, next_c, num_attacking) {
        return true;
    }

    // See if we can place a queen at (r, c).
    if num_attacking[r as usize][c as usize] == 0 {
        // Try placing a queen here and
        // recursively assigning the next square.
        board[r as usize][c as usize] = 'Q';
        num_placed += 1;

        // Increment the attack counts for this queen.
        adjust_attack_counts(num_attacking, r, c, 1);

        if do_place_queens_3(board, num_placed, next_r, next_c, num_attacking) {
            return true;
        }

        // That didn't work so remove this queen.
        board[r as usize][c as usize] = '.';
        num_placed -= 1;
        adjust_attack_counts(num_attacking, r, c, -1);
    }

    // If we get here, then there is no solution from
    // the board position before this function call.
    // Return false to backtrack and try again farther up the call stack.
    return false;
}

// Add amount to the attack counts for this square.
fn adjust_attack_counts(
    num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
    r: i32,
    c: i32,
    amount: i32,
) {
    // Attacks in the same row.
    for i in 0..INUM_COLS {
        num_attacking[r as usize][i as usize] += amount
    }

    // Attacks in the same column.
    for i in 0..INUM_ROWS {
        num_attacking[i as usize][c as usize] += amount
    }

    // Attacks in the upper left to lower right diagonal.
    for i in -INUM_ROWS..INUM_ROWS {
        let test_r = r + i;
        let test_c = c + i;
        if test_r >= 0 && test_r < INUM_ROWS && test_c >= 0 && test_c < INUM_ROWS {
            num_attacking[test_r as usize][test_c as usize] += amount;
        }
    }

    // Attacks in the upper right to lower left diagonal.
    for i in -INUM_ROWS..INUM_ROWS {
        let test_r = r + i;
        let test_c = c - i;
        if test_r >= 0 && test_r < INUM_ROWS && test_c >= 0 && test_c < INUM_ROWS {
            num_attacking[test_r as usize][test_c as usize] += amount;
        }
    }
}

pub fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    //let success = place_queens_1(&mut board, 0, 0);
    //let success = place_queens_2(&mut board, 0, 0, 0);
    let success = place_queens_3(&mut board);
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
