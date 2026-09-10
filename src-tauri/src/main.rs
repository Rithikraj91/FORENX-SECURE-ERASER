// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::drive_eraser_cmds::run_mock_drive_erasure;
use commands::file_eraser_cmds::run_mock_file_erasure;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            run_mock_drive_erasure,
            run_mock_file_erasure,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}