use std::{
    fs::read_dir,
    path::{Path, PathBuf},
    io,
};

// see reference implementations
// https://www.thorsten-hans.com/weekly-rust-trivia-get-all-files-in-a-directory/
pub fn get_filenames_from_folder(folder_path: &str) -> io::Result<Vec<String>> {
    let result = read_dir(Path::new(folder_path))?;

    let files = result
        .filter_map(|fd| {
            let p = fd.ok()?.path();
            if p.is_file() {
                p.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(files)
}

pub fn get_pathbuffers_from_folder(folder_path: &str) -> io::Result<Vec<PathBuf>> {
    let result = read_dir(Path::new(folder_path))?;

    let files = result
        .filter_map(|file| Some(file.ok()?.path()))
        .collect();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO: testing..
        assert_eq!(4, 4);
    }
}
