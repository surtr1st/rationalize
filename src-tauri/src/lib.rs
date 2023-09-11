use fs_extra::file;
use rayon::prelude::*;
use std::collections::{hash_map::DefaultHasher, hash_map::HashMap, hash_set::HashSet};
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;

const WINDOWS_EXPLORER: &str = "explorer";
const LINUX_EXPLORER: &str = "xdg-open";
const MACOS_EXPLORER: &str = "open";

pub fn read_hash_files<'rh>(target_dir: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let content = fs::read_dir(&target_dir);
    if let Ok(files) = content {
        for file in files {
            let unwrap_file = file.unwrap();
            let filename = unwrap_file.file_name().to_string_lossy().to_string();
            let hash_id = hash(&unwrap_file);
            map.insert(filename, hash_id);
        }
    }
    map
}

pub fn find_duplicates(data: &Vec<String>) {
    let mut set = HashSet::new();
    let mut duplicates = vec![];
    for item in data {
        let duplicated = set.insert(item.clone());
        if duplicated {
            duplicates.push(item.clone());
        }
    }
}

pub fn create_folder(dir: String) -> Result<String, String> {
    let result = Path::new(&dir).is_dir();
    match result {
        false => {
            Path::new(&dir);
            Ok(format!("Created folder!"))
        }
        true => Err(format!("Folder or directory exists!")),
    }
}

pub fn transfer_duplication(target_dir: String) {
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

pub fn open_location(target_dir: String) {
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

pub fn exec() {}

fn retrieve_directory_content(dir: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("should read the directory specified!")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() || path.is_dir())
        .collect::<Vec<_>>()
}

fn convert_to_linux_path(path: &str) -> String {
    let path_str = path.to_string();
    path_str.replace('\\', "/")
}

fn is_windows_path(path: &str) -> bool {
    path.contains('\\')
}

fn convert_between_linux_and_windows(path: &PathBuf) -> String {
    let path = path.to_str().unwrap();
    if is_windows_path(path) {
        return convert_to_linux_path(path);
    }
    path.to_string()
}
