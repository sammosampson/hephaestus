use std::path::PathBuf;
use std::fs::*;
use std::io::{Result, BufReader, BufRead};

pub trait FileRead: Send + Clone + 'static {
    fn read_file_to_string(&self, location: &str) -> Result<String>;
    fn read_line_from_file(&self, location: &str, line_number: usize) -> Result<String>;
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

    fn read_line_from_file(&self, location: &str, line_number: usize) -> Result<String> {
        let file = File::open(PathBuf::from(location))?;
        let reader = BufReader::new(file);
        reader.lines().nth(line_number - 1).unwrap()
    }
}