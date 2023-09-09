use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Default)]
struct Rational {
    data: Vec<String>,
}

pub fn read_hash(files: Vec<String>) {
    let mut r = Rational::default();
    for file in files {
        let hashing = hash(&file);
        r.data.push(hashing);
    }
}

pub fn compare(data: &Vec<String>) {}

pub fn create_folder() {}

pub fn transfer_duplication() {}

pub fn export_location() {}

pub fn exec() {}

fn hash<'h, T: Hash>(t: &'h T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}
