pub fn get_file_names_from_folder(folder_path: String) -> Vec<String> {
    vec!["stub".to_string(); 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
