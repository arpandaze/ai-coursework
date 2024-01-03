// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mns;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_tree() -> Vec<String> {
    return mns::State::build_breadth_first_tree();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_tree])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
