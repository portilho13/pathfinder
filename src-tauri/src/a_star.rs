use crate::structs::Pair;

struct Cell {
    parent_i: i32,
    parent_j: i32,

    f: f32,
    g: f32,
    h: f32,
}

pub type pPair =(f32, Pair);

fn is_valid(row: i32, col: i32, rows: i32, cols: i32) -> bool {
    row >= 0 && row < rows && col >= 0 && col < cols
}

fn is_unblocked(grid: Vec<Vec<i32>>, row: i32, col: i32) -> bool {
    grid[row as usize][col as usize] == 1
}

fn is_destination(row: i32, col: i32, dest: Pair) -> bool {
    row == dest.0 && col == dest.1
}

fn calculate_h_value(row: i32, col: i32, dest: Pair) -> f32 {
    // Calculate the squared differences for each dimension
    let row_diff_squared = (row - dest.0).pow(2);
    let col_diff_squared = (col - dest.1).pow(2);
    
    // Calculate the sum of squared differences and convert to f32
    let sum_squared_diff = (row_diff_squared + col_diff_squared) as f32;
    
    // Return the square root of the sum as f32
    sum_squared_diff.sqrt()
}





pub fn a_star(grid: Vec<Vec<i32>>, start: Pair, dest: Pair) {
    let rows: i32 = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;

    println!("Hello, World!");
}