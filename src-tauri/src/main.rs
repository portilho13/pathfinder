

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dijkstra;
mod a_star;

use dijkstra::dijkstra;
use a_star::a_star;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn choose_algorithm(algorithm: String) {
    match algorithm.as_str() {
        "dijkstra" => {
            // Call the dijkstra function
            dijkstra();
        }
        "aStar" => {
            // Call the a_star function
            a_star();
        }
        _ => {
            println!("Unknown algorithm: {}", algorithm);
        }
    }
}

#[tauri::command]
fn greet() {}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![choose_algorithm, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
