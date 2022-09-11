use std::path::PathBuf;
use std::fs::read_to_string;
use std::io::Result;

pub trait FileRead: Send + Clone + 'static {
    fn read_file_to_string(&self, location: &str) -> Result<String>;
}

#[derive(Clone)]
pub struct FileReader; 

pub fn create_file_reader() -> FileReader {
    FileReader
}

impl FileRead for FileReader {
    fn read_file_to_string(&self, location: &str) -> Result<String> {
        read_to_string(PathBuf::from(location))
    }
}