use std::collections::HashMap;

use crate::file_system::*;
use std::io::*;

pub struct MockFileReader(HashMap<String, String>);

impl FileRead for MockFileReader {
    fn read_file_to_string(&self, location: &str) -> Result<String> {
        if let Some(content) = self.0.get(location) {
            return Ok(content.clone());
        }
        Err(ErrorKind::NotFound.into())
    }
}

pub fn create_mock_file_reader() -> MockFileReader {
    MockFileReader(HashMap::default())
}

pub fn add_mock_file(reader: &mut MockFileReader, file_path: String, content: String) {
    reader.0.insert(file_path, content);
}