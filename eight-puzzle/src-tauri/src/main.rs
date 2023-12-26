// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod eight_puzzle;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_tree_breadth() -> Vec<String> {
    return eight_puzzle::EightPuzzleState::build_tree_depth_first(
        5,
        [2, 4, 6, 7, 3, 1, 0, 5, 8],
        // [2, 8, 3, 1, 6, 4, 7, 0, 5],
        [2, 1, 5, 4, 3, 6, 7, 8, 0], // [8, 6, 3, 2, 0, 4, 1, 7, 5],
    );
}

// #[tauri::command]
// fn generate_tree_depth() -> Vec<String> {
//     return eight_puzzle::EightPuzzleState::build_tree_depth_first(
//         [2, 8, 3, 1, 6, 4, 7, 0, 5],
//         [8, 0, 3, 2, 6, 4, 1, 7, 5],
//     );
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_tree_breadth])
        // .invoke_handler(tauri::generate_handler![generate_tree_depth])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
