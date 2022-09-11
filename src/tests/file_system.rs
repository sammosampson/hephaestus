use std::collections::HashMap;

use crate::file_system::*;
use std::io::*;

#[derive(Clone)]
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

pub fn add_mock_file(reader: &mut MockFileReader, file_path: &str, content: &str) {
    reader.0.insert(file_path.to_string(), content.to_string());
}

pub fn add_source_to_test_file_system(source: &str) -> (&str, MockFileReader) {
    let file_path = "Test.hep";
    let mut reader = create_mock_file_reader();
    add_mock_file(&mut reader, file_path, source);
    
    (file_path, reader)
}
