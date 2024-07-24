// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)]

use std::{fs, io::prelude::*, path::PathBuf};

#[tauri::command]
async fn read_file(path: PathBuf) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn write_file(
    path: PathBuf,
    content: String,
) -> Result<(), String> {
    match fs::write(path, content) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_file, write_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
