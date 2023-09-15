// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rationalize::{
    create_folder, find_duplicates, open_location, read_hash_files, transfer_duplication,
};
use std::{collections::HashMap, fs, time::Instant};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![exec, retrieve_total_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// STATUS: Untested
#[tauri::command(rename_all = "snake_case")]
fn exec(target_dir: &str) -> Result<String, String> {
    let start = Instant::now();
    let hash_files = read_hash_files(target_dir);
    if let Ok(hash_content) = hash_files {
        let duplicates = find_duplicates(&hash_content);
        if !duplicates.is_empty() {
            let transferred_folder = format!("{}/duplicates", target_dir);
            create_folder(&transferred_folder).unwrap();
            transfer_duplication(target_dir, &duplicates);
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

#[tauri::command]
fn retrieve_total_items(path: &str) -> HashMap<String, f64> {
    let mut dict = HashMap::<String, f64>::new();
    if let Ok(items) = fs::read_dir(path) {
        let items: Vec<_> = items
            .filter_map(|item| item.ok()) // Unwrap Result<DirEntry>
            .collect();

        let folders = items.iter().filter(|item| item.path().is_dir()).count();

        let files = items.iter().filter(|item| item.path().is_file()).count();

        dict.insert(String::from("folder"), folders as f64);
        dict.insert(String::from("files"), files as f64);
    }
    dict
}
