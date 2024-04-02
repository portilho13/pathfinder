use rand::{thread_rng, Rng};

pub fn recursive_division_maze(
    mut board: Vec<Vec<usize>>,
    height: i32,
    width: i32,
    row_start: i32,
    row_end: i32,
    col_start: i32,
    col_end: i32,
    surrounding_walls: bool,
) -> Vec<Vec<usize>> {
    if row_end <= row_start || col_end <= col_start {
        return board;
    }

    // Add surrounding walls if needed
    if !surrounding_walls {
        for i in 0..height {
            for j in 0..width {
                if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                    board[i as usize][j as usize] = 0;
                }
            }
        }
    }

    // Decide orientation
    let orientation = if (row_end - row_start) > (col_end - col_start) {
        "horizontal"
    } else {
        "vertical"
    };

    if orientation == "horizontal" {
        let mut possible_row: Vec<i32> = Vec::new();
        for number in (row_start..row_end).step_by(2) {
            possible_row.push(number);
        }

        let mut possible_columns: Vec<i32> = Vec::new();
        for number in (col_start - 1..col_end + 1).step_by(2) {
            possible_columns.push(number);
        }

        let mut rng = thread_rng();
        let random_row_index = rng.gen_range(0..possible_row.len());
        let random_col_index = rng.gen_range(0..possible_columns.len());
        let current_row = possible_row[random_row_index];
        let col_random = possible_columns[random_col_index];
        for i in 0..height {
            for j in 0..width {
                if i == current_row && j != col_random && j >= col_start - 1 && j <= col_end + 1 {
                    board[i as usize][j as usize] = 0;
                }
            }
        }

        let hole_row = current_row; // Choose a row to leave unblocked
        let hole_col = rng.gen_range(col_start..=col_end); // Choose a column to leave unblocked
        board[hole_row as usize][hole_col as usize] = 0; // Mark the cell as unblocked

        board = recursive_division_maze(
            board,
            height,
            width,
            row_start,
            current_row - 2,
            col_start,
            col_end,
            surrounding_walls,
        );
        board = recursive_division_maze(
            board,
            height,
            width,
            current_row + 2,
            row_end,
            col_start,
            col_end,
            surrounding_walls,
        );
    } else {
        let mut possible_columns: Vec<i32> = Vec::new();
        for number in (col_start..col_end).step_by(2) {
            possible_columns.push(number);
        }

        let mut possible_row: Vec<i32> = Vec::new();
        for number in (row_start - 1..row_end + 1).step_by(2) {
            possible_row.push(number);
        }

        let mut rng = thread_rng();
        let random_col_index = rng.gen_range(0..possible_columns.len());
        let random_row_index = rng.gen_range(0..possible_row.len());
        let current_col = possible_columns[random_col_index];
        let row_random = possible_row[random_row_index];
        for i in 0..height {
            for j in 0..width {
                if j == current_col && i != row_random && i >= row_start - 1 && i <= row_end + 1 {
                    board[i as usize][j as usize] = 0;
                }
            }
        }

        let hole_col = current_col; // Choose a column to leave unblocked
        let hole_row = rng.gen_range(row_start..=row_end); // Choose a row to leave unblocked
        board[hole_row as usize][hole_col as usize] = 0; // Mark the cell as unblocked

        board = recursive_division_maze(
            board,
            height,
            width,
            row_start,
            row_end,
            col_start,
            current_col - 2,
            surrounding_walls,
        );
        board = recursive_division_maze(
            board,
            height,
            width,
            row_start,
            row_end,
            current_col + 2,
            col_end,
            surrounding_walls,
        );
    }

    board
}
