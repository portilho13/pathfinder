use serde::de;
use std::collections::HashMap;

mod dijkstra;
mod a_star;
mod recursive_division;

use crate::recursive_division::recursive_division_maze;

use crate::dijkstra::dijkstra;
use crate::a_star::astar;

mod types {
    pub type Pair = (i32, i32);
}

use crate::types::Pair;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn choose_algorithm(algorithm: String, grid: Vec<Vec<i32>>, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> (Vec<Pair>, Vec<Pair>) {
    let start: Pair = (start_x, start_y);
    let destination: Pair = (end_x, end_y);

    match algorithm.as_str() {
        "dijkstra" => {
            if let Some((path, visited)) = dijkstra(grid, start, destination) {
                return (path, visited);
            } else {
                return (vec![], vec![]); // Returning empty vectors if no path found
            }
        }
        "aStar" => {
            if let Some((path, visited)) = astar(grid, start, destination) {
                return (path, visited); // Returning empty vector for visited nodes for A* algorithm
            } else {
                return (vec![], vec![]); // Returning empty vectors if no path found
            }
        }
        _ => {
            println!("Unknown algorithm: {}", algorithm);
            return (vec![], vec![]); // Returning empty vectors for unknown algorithm
        }
    }
}

#[tauri::command]
fn generate_grid() -> Vec<Vec<usize>> {
    let width = 50;
    let height = 20;
    let maze = recursive_division_maze(width, height);
    return maze;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![choose_algorithm, generate_grid])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
