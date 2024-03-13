use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 6;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

type Board = [[i32; NUM_COLS]; NUM_ROWS];

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(
    board: &mut Board,
    offsets: &mut [[i32; 2]; 8], // 8 possible moves, 2 coordinates each.
    cur_row: i32,
    cur_col: i32,
    num_visited: i32,
) -> bool {
    if num_visited == INUM_ROWS * INUM_COLS {
        if REQUIRE_CLOSED_TOUR {
            for offset in offsets {
                let new_row = cur_row + offset[0];
                let new_col = cur_col + offset[1];
                if new_row == 0 && new_col == 0 {
                    return true;
                }
            }
            false
        } else {
            true
        }
    } else {
        board[cur_row as usize][cur_col as usize] = num_visited;
        for offset in offsets.clone() {
            let new_row = cur_row + offset[0];
            let new_col = cur_col + offset[1];
            if is_valid(new_row, new_col)
                && board[new_row as usize][new_col as usize] == UNVISITED
                && find_tour(board, offsets, new_row, new_col, num_visited + 1)
            {
                return true;
            }
        }
        board[cur_row as usize][cur_col as usize] = UNVISITED;
        false
    }
}

fn is_valid(cur_row: i32, cur_col: i32) -> bool {
    cur_row >= 0 && cur_row < INUM_ROWS && cur_col >= 0 && cur_col < INUM_COLS
}

pub fn main() {
    // Initialize the vector of move offsets.
    let mut offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &mut offsets, 0, 0, 1);
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
fn dump_board(board: &mut [[i32; NUM_COLS]; NUM_ROWS]) {
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            print!("{:<02} ", board[r][c]);
        }
        println!();
    }
    println!();
}
