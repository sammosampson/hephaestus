use crate::{
    parsing::*,
    typing::*,
    acting::*,
    file_system::*
};

#[derive(Clone)]
pub enum CompilationMessage {
    Compile(String),
    ParseFile(String, CompilationActorHandle),
    FileParsed(FileParseResult),
    PerformTyping { unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle },
    UnitTyped(ResolvedTypes, CompilationUnit),
    FindType { criteria: FindTypeCriteria, respond_to: CompilationActorHandle },
    TypeFound(ResolvedTypeId),
    AddResolvedType(ResolvedType),
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

pub fn create_unit_typed_event(resolved_types: ResolvedTypes, unit: CompilationUnit) -> CompilationMessage {
    CompilationMessage::UnitTyped(resolved_types, unit)
}

pub fn create_type_found_event(resolved_type: ResolvedTypeId) -> CompilationMessage {
    CompilationMessage::TypeFound(resolved_type)
}

pub fn create_add_resolved_type_command(resolved_type: ResolvedType) -> CompilationMessage {
    CompilationMessage::AddResolvedType(resolved_type)
}

pub fn compile(file_name: String) {
    let (type_repository_handle, ..) = start_singleton_actor(create_type_repository_actor());
    let (compiler_handle, compiler_shutdown_notifier) = start_singleton_actor(
        create_compiler_actor(type_repository_handle)
    );
    
    send_message_to_actor(
        &compiler_handle, 
        create_compile_command(file_name)
    );

    await_shutdown(&compiler_shutdown_notifier);
}

struct CompilerActor { type_repository: CompilationActorHandle }

fn create_compiler_actor(type_repository: ActorHandle<CompilationMessage>) -> CompilerActor {
    CompilerActor { type_repository }
}


impl Actor<CompilationMessage> for CompilerActor {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::Compile(file_name) => handle_compile(file_name, ctx),
            CompilationMessage::FileParsed(parse_result) => handle_file_parsed(&self, parse_result, ctx),
            CompilationMessage::UnitTyped(resolved_types, _unit) => handle_unit_typed(&self, resolved_types),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_compile(file_name: String, ctx: &CompilationMessageContext) -> AfterReceiveAction {
    let (parser_handle, ..) = start_actor(
        ctx, 
        create_parser_actor(create_file_reader())
    );

    let compiler_handle = create_self_handle(ctx);
    
    send_message_to_actor(
        &parser_handle, 
        create_parse_file_command(file_name, compiler_handle)
    );

    continue_listening_after_receive()
}

fn handle_file_parsed(compiler: &CompilerActor, parse_result: FileParseResult, ctx: &CompilationMessageContext) -> AfterReceiveAction {
    match parse_result {
        FileParseResult::CompilationUnits { units, .. } => process_parsed_compilation_units(compiler, units, ctx),
        FileParseResult::NotFound(file_name) => process_parse_file_not_found(file_name)
    }
}

fn handle_unit_typed(compiler: &CompilerActor, resolved_types: ResolvedTypes) -> AfterReceiveAction {
    for resolved_type in resolved_types {
        send_message_to_actor(&compiler.type_repository, create_add_resolved_type_command(resolved_type));
    }
    continue_listening_after_receive()
}

fn process_parsed_compilation_units(compiler: &CompilerActor, units: CompilationUnits, ctx: &CompilationMessageContext) -> AfterReceiveAction {
    for unit in units {
        let (typing_handle, ..) = start_actor(&ctx, create_typing_actor());
        
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
