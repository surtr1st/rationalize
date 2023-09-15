use blake3::Hasher;
use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::path::PathBuf;

pub fn hash(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut hasher = Hasher::new();
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    let result = hasher.finalize();
    Ok(format!("{result}"))
}

pub fn retrieve_directory_content(dir: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("should read the directory specified!")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() || path.is_dir())
        .collect::<Vec<_>>()
}

pub fn retrieve_directory_files(dir: &str) -> Vec<DirEntry> {
    fs::read_dir(dir)
        .expect("should read the directory specified!")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .collect::<Vec<_>>()
}

pub fn convert_to_linux_path(path: &str) -> String {
    let path_str = path.to_string();
    path_str.replace('\\', "/")
}

pub fn is_windows_path(path: &str) -> bool {
    path.contains('\\')
}

pub fn convert_between_linux_and_windows(path: &PathBuf) -> String {
    let path = path.to_str().unwrap();
    if is_windows_path(path) {
        return convert_to_linux_path(path);
    }
    path.to_string()
}
