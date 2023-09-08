// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod eight_puzzle;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_tree() -> Vec<String> {
    // return mns::EightPuzzleState::build_tree();
    // return eight_puzzle::EightPuzzleState::build_tree_breadth_first([7,2,0,6,1,8,5,3,4], [1, 2, 3, 4, 5, 6, 7, 8, 0]);
    return vec![];
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_tree])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
