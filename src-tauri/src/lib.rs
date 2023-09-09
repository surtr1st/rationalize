use std::collections::{hash_map::DefaultHasher, hash_set::HashSet};
use std::hash::{Hash, Hasher};
use std::path::Path;

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

pub fn transfer_duplication() {}

pub fn export_location() {}

pub fn exec() {}

fn hash<'h, T: Hash>(t: &'h T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}
