// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rationalize::{
    create_folder, find_duplicates, open_location, read_hash_files, transfer_duplication,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![exec])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn exec(target_dir: String) -> Result<String, String> {
    let hashes = read_hash_files(&target_dir).unwrap();
    if let Ok(hash_content) = hashes {
        let duplicates = find_duplicates(&hashes);
        if !duplicates.is_empty() {
            let transferred_folder = format!("{}/duplicates", &target_dir);
            create_folder(&transferred_folder).unwrap();
            transfer_duplication(&target_dir);
            open_location(&target_dir);
        }
    }

    Ok(String::from("Successfully executed!"))
}
