use itertools::Itertools;
use walkdir::WalkDir;

pub trait FindFiles {
    fn find_files(&self, directory: &str) -> Vec<String> {
        WalkDir::new(directory)
            .into_iter()
            .filter_ok(|e| e.file_type().is_file())
            .map(|e| e.unwrap().path().to_string_lossy().to_string())
            .unique()
            .collect()
    }
}
pub struct FilesFinder {}

impl FilesFinder {
    pub fn new() -> Self {
        Self{}
    }
}

impl FindFiles for FilesFinder {}