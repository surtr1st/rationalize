// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rationalize::{
    create_folder, find_duplicates, open_location, read_hash_files, transfer_duplication,
};
use std::time::Instant;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![exec])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// TODO: Implementing folder picker dialog
#[tauri::command]
fn select_directory() {}

// STATUS: Untested
#[tauri::command]
fn exec(target_dir: &str) -> Result<String, String> {
    let start = Instant::now();
    let hash_files = read_hash_files(target_dir);
    if let Ok(hash_content) = hash_files {
        let duplicates = find_duplicates(&hash_content);
        if !duplicates.is_empty() {
            let transferred_folder = format!("{}/duplicates", target_dir);
            create_folder(&transferred_folder).unwrap();
            transfer_duplication(target_dir);
            open_location(target_dir)?;
        }
        let duration = start.elapsed();
        return Ok(format!(
            "Successfully executed! Finished in: {:?}",
            duration
        ));
    }
    Err(String::from("An error has occured!"))
}
