use std::sync::mpsc::*;

use crate::intermediate_representation::*;
use crate::compilation::*;
use crate::testing::*;
use crate::parsing::*;
use crate::threading::*;
use crate::typing::*;
use crate::types::*;
use crate::acting::*;
use crate::errors::*;


pub fn compile_source_and_get_intemediate_representation(source: &str) -> Vec<IntermediateRepresentation> {
    let (file_path, reader) = add_source_to_test_file_system(source);    
    compile_file_and_get_intemediate_representation(file_path, reader)
}

pub fn compile_file_and_get_intemediate_representation(file_path: &str, reader: MockFileReader) -> Vec<IntermediateRepresentation> {
    let message_receiver = compile_and_get_message_receiver(file_path, reader);
    
    let mut result = vec!();

    loop {
        let next_message = message_receiver.recv().unwrap();
        match next_message {
            CompilationMessage::ByteCodeBuilt { code, .. } => result.push(code),
            CompilationMessage::CompilationComplete => break,           
            _ => {}
        }
    }

    return result
}

pub fn get_first_ir_with_byte_code_named<'a>(irs: &'a Vec<IntermediateRepresentation>, name: &str) -> &'a IntermediateRepresentation {
    irs
        .iter()
        .filter(|ir| ir.byte_code.len() > 0 && ir.top_level_symbol == name)
        .next()
        .unwrap()
}

pub fn get_first_ir_named<'a>(irs: &'a Vec<IntermediateRepresentation>, name: &str) -> &'a IntermediateRepresentation {
    irs
        .iter()
        .filter(|ir| ir.top_level_symbol == name)
        .next()
        .unwrap()
}

pub fn compile_and_get_message_receiver(file_path: &str, file_reader: MockFileReader) -> Receiver<CompilationMessage> {    
    let (message_sender, message_receiver) = channel::<CompilationMessage>();
    let message_wire_tap = create_send_message_wire_tap(message_sender);
    let interpreter = create_test_backend();
    compile(file_path.to_string(), file_reader, interpreter, message_wire_tap);
    message_receiver
}

pub struct SendMessageWireTap {
    sender: Sender<CompilationMessage>
}

fn create_send_message_wire_tap(sender: Sender<CompilationMessage>) -> SendMessageWireTap {
    SendMessageWireTap { sender }
}

impl WireTapCompilationMessage for SendMessageWireTap {
    fn tap(&mut self, message: &CompilationMessage) {
        self.sender.send(message.clone()).unwrap();
    }
}

pub fn compile_source_and_get_types_and_unit(source: &str) -> Vec<(CompilationUnit, RuntimeTypePointers, CompilationErrors)> {
    let (file_path, reader) = add_source_to_test_file_system(source);    
    compile_file_and_get_types_and_unit(file_path, reader)
}

fn compile_file_and_get_types_and_unit(file_path: &str, reader: MockFileReader) -> Vec<(CompilationUnit, RuntimeTypePointers, CompilationErrors)> {
    let message_receiver = compile_and_get_message_receiver(file_path, reader);
    
    let mut result = vec!();

    loop {
        let next_message = message_receiver.recv().unwrap();
        match next_message {
            CompilationMessage::UnitTyped { resolved_types, unit, errors } => result.push((unit, resolved_types, errors)),
            CompilationMessage::CompilationComplete => break,           
            _ => {}
        }
    }

    return result
}


pub fn compile_source_and_get_parsed_units_and_errors(source: &str) -> Vec<(CompilationUnits, CompilationErrors)> {
    let (file_path, reader) = add_source_to_test_file_system(source);    
    compile_file_and_get_parsed_units_and_errors(file_path, reader)
}

fn compile_file_and_get_parsed_units_and_errors(file_path: &str, reader: MockFileReader) -> Vec<(CompilationUnits, CompilationErrors)> {
    let message_receiver = compile_and_get_message_receiver(file_path, reader);
    
    let mut result = vec!();

    loop {
        let next_message = message_receiver.recv().unwrap();
        match next_message {
            CompilationMessage::FileParsed(parse_result) => {
                if let FileParseResult::CompilationUnits { units, errors, .. } = parse_result {
                    result.push((units, errors));
                }
            },
            CompilationMessage::CompilationComplete => break,           
            _ => {}
        }
    }

    return result
}


pub fn get_first_typed_const_unit(units_and_types: &[(CompilationUnit, RuntimeTypePointers, CompilationErrors)]) -> &(CompilationUnit, RuntimeTypePointers, CompilationErrors) {
    units_and_types
        .iter()
        .filter(|(unit, _, _)| {
            match unit.tree.item_ref() {
                AbstractSyntaxNodeItem::Constant { .. } => true,
                _ => false,
            }
        })
        .next()
        .unwrap()
}

pub fn get_first_typed_procedure_body_unit(units_and_types: &[(CompilationUnit, RuntimeTypePointers, CompilationErrors)]) -> &(CompilationUnit, RuntimeTypePointers, CompilationErrors) {
    units_and_types
        .iter()
        .filter(|(unit, _, _)| {
            match unit.tree.item_ref() {
                AbstractSyntaxNodeItem::ProcedureBody { .. } => true,
                _ => false,
            }
        })
        .next()
        .unwrap()
}

pub fn run_typing_on_unit(typing_repository: CompilationActorHandle, unit: CompilationUnit) -> (RuntimeTypePointers, CompilationUnit) {
    let (
        message_receiver_handle, 
        message_receiver
    ) = create_test_message_receiver_actor();
    
    let (typing_actor, ..) = start_singleton_actor(create_typing_actor());
        
    send_message_to_actor(
        &typing_actor, 
        create_perform_typing_command(unit, typing_repository, message_receiver_handle)
    );

    let next_message = message_receiver.into_iter().next().unwrap();

    let result = match next_message {
        CompilationMessage::UnitTyped { resolved_types, unit, .. } => Some((resolved_types, unit)),
        _ => None
    };

    result.unwrap()
}

pub fn start_type_repository_actor() -> CompilationActorHandle {
    let (handle, _) = start_singleton_actor(create_type_repository_actor());
    handle
}

pub fn add_resolved_type(typing_repository: &CompilationActorHandle, resolved_type: RuntimeType) {
    send_message_to_actor(
        typing_repository, 
        create_add_resolved_type_command(create_shareable(resolved_type))
    );
}

pub fn create_procedure_definition_type(name: &str, arg_types: RuntimeTypePointers, return_types: RuntimeTypePointers) -> RuntimeType {
    let other_proc_type = create_type(
        user_defined_runtime_type_id(create_compilation_unit_id()), 
        name.to_string(),
        procedure_definition_type_item(arg_types, return_types),
        not_required_type_size()
    );
    other_proc_type
}

pub fn create_procedure_definition_type_with_no_args(name: &str) -> RuntimeType {
    create_procedure_definition_type(
        name,
        vec!(), 
        vec!()
    )
}