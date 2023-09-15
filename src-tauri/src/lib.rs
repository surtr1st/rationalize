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
use std::sync::{Arc, Mutex, RwLock};

const PARALLEL_THRESHOLD: usize = 50;
const WINDOWS_EXPLORER: &str = "explorer";
const LINUX_EXPLORER: &str = "xdg-open";
const MACOS_EXPLORER: &str = "open";

pub fn read_hash_files(target_dir: &str) -> Result<HashMap<String, String>, String> {
    let map = Arc::new(Mutex::new(HashMap::new()));
    let files = retrieve_directory_files(&target_dir);

    if files.len() <= PARALLEL_THRESHOLD {
        for file in files {
            let mut map = map.lock().unwrap();
            let item = file.path();
            let file_path = item.to_string_lossy().to_string();
            let file_name = item.file_name().unwrap().to_string_lossy();

            let hash_content = match hash(&file_path) {
                Ok(content) => content,
                Err(hashing_error) => panic!("{hashing_error}"),
            };
            map.insert(file_name.into_owned(), hash_content);
        }
    } else {
        files.par_iter().for_each(|file| {
            let mut map = map.lock().unwrap();
            let item = file.path();
            let file_path = item.to_string_lossy().to_string();
            let file_name = item.file_name().unwrap().to_string_lossy();

            let hash_content = match hash(&file_path) {
                Ok(content) => content,
                Err(hashing_error) => panic!("{hashing_error}"),
            };
            map.insert(file_name.into_owned(), hash_content);
        })
    }

    Ok(Arc::try_unwrap(map).unwrap().into_inner().unwrap())
}

pub fn find_duplicates(data: &HashMap<String, String>) -> HashMap<String, String> {
    let unique_values_set = Arc::new(RwLock::new(HashSet::new()));
    let duplicates = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    if data.len() <= PARALLEL_THRESHOLD {
        for (key, value) in data {
            let mut unique_values_set = unique_values_set.write().unwrap();
            let mut duplicates = duplicates.lock().unwrap();

            if !unique_values_set.insert(value.as_str()) {
                duplicates.insert(key.to_string(), value.to_string());
            }
        }
    } else {
        data.par_iter().for_each(|(key, value)| {
            let mut unique_values_set = unique_values_set.write().unwrap();
            let mut duplicates = duplicates.lock().unwrap();

            if !unique_values_set.insert(value.as_str()) {
                duplicates.insert(key.to_string(), value.to_string());
            }
        });
    }

    Arc::try_unwrap(duplicates).unwrap().into_inner().unwrap()
}

pub fn create_folder(dir: &str) -> Result<(), Error> {
    if !Path::new(dir).is_dir() {
        fs::create_dir(dir)?;
    }
    Ok(())
}

pub fn transfer_duplication(target_dir: &str, duplicates: &HashMap<String, String>) {
    let duplicates_dir = Path::new(&target_dir).join("duplicates");
    let dir_content = retrieve_directory_content(&target_dir);

    let files: Vec<_> = dir_content
        .par_iter()
        .filter(|item| item.is_file())
        .collect();

    if files.len() <= PARALLEL_THRESHOLD {
        files.iter().for_each(|item| {
            let filename = item
                .file_name()
                .unwrap_or_else(|| panic!("should return file: {}", item.to_str().unwrap()))
                .to_string_lossy()
                .to_string();

            if duplicates.contains_key(&filename) {
                let destination = Path::new(&duplicates_dir).join(filename);
                let options = file::CopyOptions::new();
                file::move_file(
                    &convert_between_linux_and_windows(&item),
                    &destination,
                    &options,
                )
                .unwrap_or_else(|_| panic!("should transfer to dir: {}", &destination.display()));
            }
        });
    } else {
        files.par_iter().for_each(|item| {
            let filename = item
                .file_name()
                .unwrap_or_else(|| panic!("should return file: {}", item.to_str().unwrap()))
                .to_string_lossy()
                .to_string();

            if duplicates.contains_key(&filename) {
                let destination = Path::new(&duplicates_dir).join(filename);
                let options = file::CopyOptions::new();
                file::move_file(
                    &convert_between_linux_and_windows(&item),
                    &destination,
                    &options,
                )
                .unwrap_or_else(|_| panic!("should transfer to dir: {}", &destination.display()));
            }
        });
    }
}

pub fn open_location(target_dir: &str) -> Result<String, String> {
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
                return Ok(String::from("File explorer opened successfully."));
            }
            return Err(String::from("Failed to open file explorer."));
        }
        Err(err) => panic!("Error opening file explorer: {:?}", err),
    }
}
