mod procedures;
mod constants;
mod assignments;

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

    loop {
        let next_message = message_receiver.recv().unwrap();
        match next_message {
            CompilationMessage::ByteCodeBuilt { code } => result.push(code),
            CompilationMessage::CompilationComplete => break,           
            _ => {}
        }
    }

    return result
}

fn get_first_ir_with_byte_code_named<'a>(irs: &'a Vec<IntermediateRepresentation>, name: &str) -> &'a IntermediateRepresentation {
    irs
        .iter()
        .filter(|ir| ir.byte_code.len() > 0 && ir.top_level_symbol == name)
        .next()
        .unwrap()
}

fn get_first_ir_named<'a>(irs: &'a Vec<IntermediateRepresentation>, name: &str) -> &'a IntermediateRepresentation {
    irs
        .iter()
        .filter(|ir| ir.top_level_symbol == name)
        .next()
        .unwrap()
}