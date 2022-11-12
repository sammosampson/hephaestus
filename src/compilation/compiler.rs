use crate::{
    compilation::*,
    typing::*,
    acting::*,
    file_system::*,
    backends::*,
    errors::*,
};

pub fn compile<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    file_name: String,
    reader: TReader,
    backend: TBackend,
    message_wire_tap: TMessageWireTap
) {
    let type_repository = create_type_repository_actor();
    let error_reporter = create_error_reporter_actor();
    let (type_repository_handle, ..) = start_singleton_actor(type_repository);
    let (error_reporter_handle, ..) = start_singleton_actor(error_reporter);
    let (compiler_handle, compiler_shutdown_notifier) = start_singleton_actor(
        create_compiler_actor(type_repository_handle, error_reporter_handle, reader, backend, message_wire_tap)
    );

    start_compilation(compiler_handle, file_name);

    await_shutdown(&compiler_shutdown_notifier);
}

fn start_compilation(compiler_handle: ActorHandle<CompilationMessage>, file_name: String) {
    send_message_to_actor(
        &compiler_handle, 
        create_compile_command(file_name)
    );
}

pub struct CompilerActor<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> {
    pub statistics: Statistics,
    pub type_repository: CompilationActorHandle,
    pub error_reporter: CompilationActorHandle,
    pub reader: TReader,
    pub backend: TBackend,
    pub message_wire_tap: TMessageWireTap,
    pub errors_have_occurred: bool
}

fn create_compiler_actor<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    type_repository: CompilationActorHandle,
    error_reporter: CompilationActorHandle,
    reader: TReader,
    backend: TBackend, 
    message_wire_tap: TMessageWireTap
) -> CompilerActor<TReader, TBackend, TMessageWireTap> {
    CompilerActor {
        statistics: create_statistics(),
        type_repository,
        error_reporter, 
        reader,
        backend,
        message_wire_tap,
        errors_have_occurred: false
    }
}

impl <TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> Actor<CompilationMessage> for CompilerActor<TReader, TBackend, TMessageWireTap> {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        self.message_wire_tap.tap(&message);
        match message {
            CompilationMessage::Compile(file_name) =>
                handle_compile(self, ctx, file_name ),
            CompilationMessage::FileParsed { units, file_name } =>
                handle_file_parsed(self, file_name, units, ctx),
            CompilationMessage::UnitTyped { resolved_types, unit } => 
                handle_unit_typed(self, unit, resolved_types, ctx),
            CompilationMessage::UnitSized { unit } => 
                handle_unit_sized(self, unit, ctx),
            CompilationMessage::ByteCodeBuilt { code, unit  } => 
                handle_byte_code_built(self, unit, code, ctx, self.backend.clone()),
            CompilationMessage::BackendBuilt { id, .. } => 
                handle_backend_built(self, id, ctx),
            CompilationMessage::ErrorsReported => 
                handle_errors_reported(self),
            CompilationMessage::CompilationComplete =>
                handle_compilation_complete(self),
            _ => continue_listening_after_receive()
        }
    }
}

pub fn handle_compile<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    ctx: &CompilationMessageContext,
    file_name: String,
) -> AfterReceiveAction {
    parse_file(compiler, file_name, ctx);
    continue_listening_after_receive()
}

pub fn handle_compilation_complete<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
) -> AfterReceiveAction {
    shut_down(&compiler.type_repository);
    shut_down(&compiler.error_reporter);
    shutdown_after_receive()
}

fn shut_down(actor: &CompilationActorHandle) {
    send_message_to_actor(actor, create_shutdown_command());
}

