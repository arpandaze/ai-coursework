#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod eight_puzzle;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_tree_breadth() -> Vec<String> {
    let (ret, _) = eight_puzzle::EightPuzzleState::build_tree_depth_first(
        true,
        8,
        [1, 2, 3, 8, 0, 4, 7, 6, 5],
        [8, 0, 3, 2, 6, 4, 1, 7, 5],
    );

    return ret;
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
