use std::path::PathBuf;
use std::fs::read_to_string;
use std::io::Result;

pub fn read_file_to_string(location: &str) -> Result<String> {
    read_to_string(PathBuf::from(location))
}