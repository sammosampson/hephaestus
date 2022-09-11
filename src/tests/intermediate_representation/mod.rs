mod expressions;

use crate::{
    intermediate_representation::*,
    compilation::*,
    tests::file_system::*,
    tests::compilation::*
};


fn compile_source_and_get_intemediate_representation(source: &str) -> Vec<IntermediateRepresentation> {
    let (file_path, reader) = add_source_to_test_file_system(source);    
    compile_file_and_get_intemediate_representation(file_path, reader)
}

fn compile_file_and_get_intemediate_representation(file_path: &str, reader: MockFileReader) -> Vec<IntermediateRepresentation> {
    let message_receiver = compile_and_get_message_receiver(file_path, reader);
    
    let mut result = vec!();

    for next_message in message_receiver {
        match next_message {
            CompilationMessage::ByteCodeAssembled { code } => result.push(code),
            CompilationMessage::CompilationComplete => break,           
            _ => {}
        }
    }

    return result
}