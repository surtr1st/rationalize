use std::collections::{hash_map::DefaultHasher, hash_set::HashSet};
use std::hash::{Hash, Hasher};

pub fn read_hash(files: Vec<String>) {
    let mut hashes = vec![];
    for file in files {
        let hashing = hash(&file);
        hashes.push(hashing);
    }
}

pub fn compare(data: &Vec<String>) {
    let mut set = HashSet::new();
    let mut duplicates = vec![];
    for item in data {
        let duplicated = set.insert(item.clone());
        if duplicated {
            duplicates.push(item.clone());
        }
    }
}

pub fn create_folder() {}

pub fn transfer_duplication() {}

pub fn export_location() {}

pub fn exec() {}

fn hash<'h, T: Hash>(t: &'h T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}
