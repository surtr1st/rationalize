pub mod helpers;
use fs_extra::file;
use helpers::*;
use rayon::prelude::*;
use std::collections::{hash_map::HashMap, hash_set::HashSet};
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

const WINDOWS_EXPLORER: &str = "explorer";
const LINUX_EXPLORER: &str = "xdg-open";
const MACOS_EXPLORER: &str = "open";

fn read_hash_files(target_dir: &str) -> Result<HashMap<String, String>, Error> {
    let mut map = HashMap::new();
    let files = fs::read_dir(&target_dir)?;
    for item in files {
        let file = item?;
        let path = file.path();
        let name = file.file_name();

        let filepath = path.to_string_lossy();
        let filename = name.to_string_lossy();
        let hash_id = hash(&filepath)?;

        map.insert(filename.into_owned(), hash_id);
    }
    Ok(map)
}

fn find_duplicates(data: &HashMap<String, String>) -> Vec<&str> {
    let mut unique_values_set = HashSet::new();
    let mut duplicates = vec![];
    for (key, value) in data {
        if !unique_values_set.insert(value.as_str()) {
            duplicates.push(key.as_str());
        }
    }
    duplicates
}

fn create_folder(dir: &str) -> Result<(), Error> {
    if !Path::new(dir).is_dir() {
        fs::create_dir(dir)?;
    }
    Ok(())
}

fn transfer_duplication(target_dir: &str) {
    retrieve_directory_content(&target_dir)
        .par_iter()
        .for_each(|item| {
            let filename = item
                .file_name()
                .unwrap_or_else(|| panic!("should return file: {}", item.to_str().unwrap()));
            if item.is_file() {
                let destination = format!("{}/{}", &target_dir, filename.to_str().unwrap());
                let options = file::CopyOptions::new();
                file::move_file(
                    &convert_between_linux_and_windows(&item),
                    &destination,
                    &options,
                )
                .unwrap_or_else(|_| panic!("should transfer files to {}", &destination));
            }
        })
}

fn open_location(target_dir: &str) {
    let mut cmd = Command::new("");
    let current_os = env::consts::OS;
    if current_os == "windows" {
        cmd = Command::new(WINDOWS_EXPLORER);
    }
    if current_os == "linux" {
        cmd = Command::new(LINUX_EXPLORER);
    }
    if current_os == "macos" {
        cmd = Command::new(MACOS_EXPLORER);
    }
    cmd.arg(&target_dir);

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("File explorer opened successfully.");
            } else {
                eprintln!("Failed to open file explorer.");
            }
        }
        Err(err) => {
            eprintln!("Error opening file explorer: {:?}", err);
        }
    }
}

pub fn exec(target_dir: String) -> Result<String, Error> {
    let hashes = read_hash_files(&target_dir)?;
    let duplicates = find_duplicates(&hashes);
    if !duplicates.is_empty() {
        create_folder("./duplicates")?;
        transfer_duplication(&target_dir);
        open_location(&target_dir);
    }
    Ok(String::from("Successfully executed!"))
}
