use fs_extra::{dir, file};
use rayon::prelude::*;
use std::collections::{hash_map::DefaultHasher, hash_set::HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub fn read_hash<'rh>(files: &Vec<String>) -> Vec<String> {
    let mut hashes = vec![];
    for file in files {
        let hashing = hash(&file);
        hashes.push(hashing);
    }
    hashes
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
        true => Ok(format!("Created folder!")),
        false => Err(format!("Folder or directory exists!")),
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

pub fn export_location() {}

pub fn exec() {}

fn hash<'h, T: Hash>(t: &'h T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}

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
