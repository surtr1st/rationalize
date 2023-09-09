pub mod rationalize {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[tauri::command]
    pub fn read_hash(data: &Vec<String>) {}

    #[tauri::command]
    pub fn compare(data: &Vec<String>) {}

    #[tauri::command]
    pub fn create_folder() {}

    #[tauri::command]
    pub fn transfer_duplication() {}

    #[tauri::command]
    pub fn export_location() {}

    #[tauri::command]
    pub fn exec() {}

    pub fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
