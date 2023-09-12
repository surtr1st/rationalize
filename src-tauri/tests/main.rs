#[cfg(test)]
mod rationalize_on_action {
    use rationalize::read_hash_files;

    #[test]
    fn try_read_hash_from_input() {
        let result = read_hash_files("./".to_string());
        if let Ok(content) = result {
            assert!(!content.is_empty());
        }
    }

    #[test]
    fn try_find_duplicate_values() {}

    #[test]
    fn try_create_folder_or_directory() {}

    #[test]
    fn try_transfer_duplicates() {}

    #[test]
    fn try_open_location() {}
}
