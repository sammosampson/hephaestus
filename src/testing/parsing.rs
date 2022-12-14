use crate::parsing::*;
use crate::compilation::*;
use crate::testing::*;
use crate::acting::*;

pub fn run_parse_file(file_path: &str, content: &str) -> (String, CompilationUnits) {
    let mut reader = create_mock_file_reader();
    add_mock_file(&mut reader, file_path, content);

    let (message_receiver_handle, message_receiver) = create_test_message_receiver_actor();
    let (error_reporter, ..) = create_test_message_receiver_actor();
    
    let (parser, ..) = start_singleton_actor(create_parser_actor(message_receiver_handle, error_reporter, reader));
    send_message_to_actor(&parser, create_parse_file_command(file_path.to_string()));

    let next_message = message_receiver.into_iter().next().unwrap();

    let (actual_file_path, units) = match next_message {
        CompilationMessage::FileParsed { file_name, units, .. } => (file_name, units),
        _ => (String::default(), vec!())
    };

    (actual_file_path, units)
}

pub fn run_parse_file_return_only_units(content: &str) -> CompilationUnits {
    let (_actual_file_path, units) = run_parse_file(
        "test.hep", 
        content
    );

    units
}
