use std::collections::HashMap;

use crate::{
    parsing::*,
    typing::*,
    acting::*,
    file_system::*,
    intermediate_representation::*
};

#[derive(Clone)]
pub enum CompilationMessage {
    Compile(String),
    ParseFile(String, CompilationActorHandle),
    FileParsed(FileParseResult),
    PerformTyping { unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle },
    UnitTyped(RuntimeTypePointers, CompilationUnit),
    FindType { criteria: FindTypeCriteria, respond_to: CompilationActorHandle },
    TypeFound(RuntimeTypePointer),
    AddResolvedType(RuntimeTypePointer),
    AssembleByteCode{ unit: CompilationUnit, compiler: CompilationActorHandle },
    ByteCodeAssembled{ code: IntermediateRepresentation },
    RunByteCode{ code: ByteCodeInstructionStream },
    CompilationComplete
}

pub trait WireTapCompilationMessage : Send + 'static {
    fn tap(&mut self, message: &CompilationMessage);
}

pub struct NullCompilationMessageWireTap;

impl WireTapCompilationMessage for NullCompilationMessageWireTap {
    fn tap(&mut self, _message: &CompilationMessage) {
    }
}

pub fn create_null_message_wire_tap() -> NullCompilationMessageWireTap {
    NullCompilationMessageWireTap
}

pub type CompilationActorHandle = ActorHandle<CompilationMessage>;
pub type CompilationMessageContext = ActorContext<CompilationMessage>;

fn create_compile_command(file_name: String) -> CompilationMessage {
    CompilationMessage::Compile(file_name)
}

pub fn create_parse_file_command(file_name: String, handle: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::ParseFile(file_name, handle)
}

pub fn create_perform_typing_command(unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::PerformTyping { unit, type_repository, compiler }
}

pub fn create_find_type_request(criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::FindType { criteria, respond_to }
}

pub fn create_file_parsed_event(parse_result: FileParseResult) -> CompilationMessage {
    CompilationMessage::FileParsed(parse_result)
}

pub fn create_unit_typed_event(resolved_types: RuntimeTypePointers, unit: CompilationUnit) -> CompilationMessage {
    CompilationMessage::UnitTyped(resolved_types, unit)
}

pub fn create_type_found_event(resolved_type: RuntimeTypePointer) -> CompilationMessage {
    CompilationMessage::TypeFound(resolved_type)
}

pub fn create_add_resolved_type_command(resolved_type: RuntimeTypePointer) -> CompilationMessage {
    CompilationMessage::AddResolvedType(resolved_type)
}

pub fn create_assemble_bytecode_command(unit: CompilationUnit, compiler: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::AssembleByteCode { unit, compiler }
}

pub fn create_bytecode_assembled_event(code: IntermediateRepresentation) -> CompilationMessage {
    CompilationMessage::ByteCodeAssembled { code }
}

fn create_compilation_complete_event() -> CompilationMessage {
    CompilationMessage::CompilationComplete
}

pub fn try_get_type_found_compilation_message(message: CompilationMessage) -> Option<RuntimeTypePointer> {
    if let CompilationMessage::TypeFound(resolved_type) = message {
       return Some(resolved_type);
    }
    None
}

pub fn compile<TReader: FileRead, TMessageWireTap: WireTapCompilationMessage>(
    file_name: String,
    reader: TReader,
    message_wire_tap: TMessageWireTap
) {
    let (type_repository_handle, ..) = start_singleton_actor(create_type_repository_actor());
    let (compiler_handle, compiler_shutdown_notifier) = start_singleton_actor(
        create_compiler_actor(type_repository_handle, reader, message_wire_tap)
    );
    
    send_message_to_actor(
        &compiler_handle, 
        create_compile_command(file_name)
    );

    await_shutdown(&compiler_shutdown_notifier);
}

type CompilationUnitsRequestedList = HashMap<CompilationUnitId, CompilationUnitId>;

fn create_compilation_units_requested_list() -> HashMap<CompilationUnitId, CompilationUnitId> {
    HashMap::default()
}

fn register_compilation_requested(lookup: &mut CompilationUnitsRequestedList, id: CompilationUnitId) {
    lookup.insert(id, id);
}

fn remove_unit_from_compilation_requested_list(lookup: &mut CompilationUnitsRequestedList, id: &CompilationUnitId) {
    lookup.remove(id);
}

fn compilation_requested_list_is_empty(lookup: &CompilationUnitsRequestedList) -> bool {
    lookup.is_empty()
}

struct CompilerActor<TReader: FileRead, TMessageWireTap: WireTapCompilationMessage> {
    compilation_units_requested_list: CompilationUnitsRequestedList,
    type_repository: CompilationActorHandle,
    reader: TReader,
    message_wire_tap: TMessageWireTap
}

fn create_compiler_actor<TReader: FileRead, TMessageWireTap: WireTapCompilationMessage>(
    type_repository: CompilationActorHandle,
    reader: TReader,
    message_wire_tap: TMessageWireTap
) -> CompilerActor<TReader, TMessageWireTap> {
    CompilerActor {
        compilation_units_requested_list: create_compilation_units_requested_list(),
        type_repository,
        reader,
        message_wire_tap
    }
}

impl <TReader: FileRead, TMessageWireTap: WireTapCompilationMessage> Actor<CompilationMessage> for CompilerActor<TReader, TMessageWireTap> {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        self.message_wire_tap.tap(&message);
        
        match message {
            CompilationMessage::Compile(file_name) =>
                handle_compile(file_name, ctx, self.reader.clone()),
            CompilationMessage::FileParsed(parse_result) =>
                handle_file_parsed(self, parse_result, ctx),
            CompilationMessage::UnitTyped(resolved_types, unit) => 
                handle_unit_typed(self, unit, resolved_types, ctx),
            CompilationMessage::ByteCodeAssembled { code } => 
                handle_byte_code_assembled(self, code, ctx),
            CompilationMessage::CompilationComplete => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_compile<TReader: FileRead>(
    file_name: String,
    ctx: &CompilationMessageContext,
    reader: TReader
) -> AfterReceiveAction {
    let (parser_handle, ..) = start_actor(
        ctx, 
        create_parser_actor(reader)
    );

    let compiler_handle = create_self_handle(ctx);
    
    send_message_to_actor(
        &parser_handle, 
        create_parse_file_command(file_name, compiler_handle)
    );

    continue_listening_after_receive()
}

fn handle_file_parsed<TReader: FileRead, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TMessageWireTap>,
    parse_result: FileParseResult,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    match parse_result {
        FileParseResult::CompilationUnits { units, .. } => process_parsed_compilation_units(compiler, units, ctx),
        FileParseResult::NotFound(file_name) => process_parse_file_not_found(file_name)
    }
}

fn handle_unit_typed<TReader: FileRead, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &CompilerActor<TReader, TMessageWireTap>, 
    unit: CompilationUnit,
    resolved_types: RuntimeTypePointers,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    for resolved_type in resolved_types {
        send_message_to_actor(&compiler.type_repository, create_add_resolved_type_command(resolved_type));
    }
    
    let (intemediate_representation_handle, ..) = start_actor(
        ctx, 
        create_intemediate_representation_actor()
    );

    let compiler_handle = create_self_handle(&ctx);

    send_message_to_actor(
        &intemediate_representation_handle, 
        create_assemble_bytecode_command(unit, compiler_handle)
    );

    continue_listening_after_receive()
}

fn handle_byte_code_assembled<TReader: FileRead, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TMessageWireTap>,
    code: IntermediateRepresentation,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    remove_unit_from_compilation_requested_list(
        &mut compiler.compilation_units_requested_list,
        &code.id
    );

    if compilation_requested_list_is_empty(&compiler.compilation_units_requested_list) {
        send_message_to_actor(&create_self_handle(ctx), create_compilation_complete_event());
    }

    continue_listening_after_receive()
}

fn process_parsed_compilation_units<TReader: FileRead, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TMessageWireTap>,
    units: CompilationUnits,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    for unit in units {

        register_compilation_requested(&mut compiler.compilation_units_requested_list, unit.id);

        let (typing_handle, ..) = start_actor(
            &ctx, 
            create_typing_actor()
        );
        
        send_message_to_actor(
            &typing_handle, 
            create_perform_typing_command(
                unit, 
                compiler.type_repository.clone(), 
                create_self_handle(ctx)
            )
        );
    }

    continue_listening_after_receive()
}

fn process_parse_file_not_found(file_name: String) -> AfterReceiveAction {
    println!("{} not found", file_name);
    shutdown_after_receive()
}
