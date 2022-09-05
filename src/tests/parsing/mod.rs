mod directives;
mod consts;
mod procedures;

use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;
use crate::tests::file_system::*;
use crate::tests::acting::*;

pub fn run_parse_file(file_path: &str, content: &str) -> (String, Vec<CompilationUnit>) {
    let mut reader = create_mock_file_reader();
    add_mock_file(&mut reader, file_path.to_string(), content.to_string());

    let (message_receiver_handle, message_receiver) = create_test_message_receiver_handle();
    
    let (parser, ..) = start_singleton_actor(create_parser_actor(reader));
    send_message_to_actor(&parser, create_parse_file_command(file_path.to_string(), message_receiver_handle));

    let next_message = message_receiver.into_iter().next().unwrap();

    let (actual_file_path, units) = match next_message {
        CompilationMessage::FileParsed(FileParseResult::CompilationUnits { file_name, units }) => (file_name, units),
        _ => (String::default(), vec!())
    };

    (actual_file_path, units)
}

#[test]
fn parse_empty_input_parses_correctly() {
    let file_path = "test.hep";
    
    let (actual_file_path, units, ..) = run_parse_file(
        file_path, 
        ""
    );
       
    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(0, units.len());
}