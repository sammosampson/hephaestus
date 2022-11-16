use std::collections::HashMap;
use log::*;

use crate::{
    compilation::*,
    parsing::*,
    acting::*,
};

pub type FilesInPipe = HashMap<String, String>;
pub type UnitsInPipe = HashMap<CompilationUnitId, CompilationUnitId>;
pub type AwaitedUnitsInPipe = HashMap<CompilationUnitId, CompilationUnitId>;

pub struct Statistics {
    files_in_pipe: FilesInPipe,
    units_in_pipe: UnitsInPipe,
    awaited_units_in_pipe: AwaitedUnitsInPipe
}

pub fn create_statistics() -> Statistics {
    Statistics {
        files_in_pipe: HashMap::default(),
        units_in_pipe: HashMap::default(),
        awaited_units_in_pipe: HashMap::default(),
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum CompilationPhase{
    Parsing(String),
    Typing(CompilationUnitId),
    Sizing(CompilationUnitId),
    ByteCodeCreation(CompilationUnitId),
    BackendBuild(CompilationUnitId)
}

pub fn parsing_compilation_phase(filename: String) -> CompilationPhase {
    CompilationPhase::Parsing(filename)
}

pub fn typing_compilation_phase(id: CompilationUnitId) -> CompilationPhase {
    CompilationPhase::Typing(id)
}

pub fn sizing_compilation_phase(id: CompilationUnitId) -> CompilationPhase {
    CompilationPhase::Sizing(id)
}

pub fn byte_code_creation_compilation_phase(id: CompilationUnitId) -> CompilationPhase {
    CompilationPhase::ByteCodeCreation(id)
}

pub fn backend_build_compilation_phase(id: CompilationUnitId) -> CompilationPhase {
    CompilationPhase::BackendBuild(id)
}

pub fn start_compilation_phase(statistics: &mut Statistics, phase: CompilationPhase) {
    log_start_compilation_phase(&phase);
    perform_start_compilation_phase(statistics, phase);
    log_statistics(statistics);
}

pub fn end_compilation_phase(statistics: &mut Statistics, type_repository: &CompilationActorHandle, phase: CompilationPhase, ctx: &CompilationMessageContext) {
    log_end_compilation_phase(&phase);
    perform_end_compilation_phase(statistics, type_repository, phase, ctx);
    log_statistics(statistics);
}

pub fn await_unit_in_statistics(statistics: &mut Statistics, type_repository: &CompilationActorHandle, awaited_unit_id: CompilationUnitId) {
    add_awaited_unit_to_statistics(statistics, awaited_unit_id);
    circuit_break_awaited_units_if_required(statistics, type_repository);
}

fn perform_start_compilation_phase(statistics: &mut Statistics, phase: CompilationPhase) {
    match phase {
        CompilationPhase::Parsing(filename) => 
            add_file_to_statistics(statistics, filename),
        CompilationPhase::Typing(id) => 
            add_unit_to_statistics(statistics, id),
        _ => {},
    }
}

fn perform_end_compilation_phase(statistics: &mut Statistics, type_repository: &CompilationActorHandle, phase: CompilationPhase, ctx: &CompilationMessageContext) {
    match phase {
        CompilationPhase::Parsing(filename) => {
            remove_file_from_statistics(statistics, &filename);
            all_files_parsed(statistics);
        },
        CompilationPhase::Typing(id) =>
            remove_awaited_unit_from_statistics(statistics, &id),
        CompilationPhase::BackendBuild(id) => {
            remove_unit_from_statistics(statistics, &id);
            circuit_break_awaited_units_if_required(statistics, type_repository);
            check_for_statistics_completion(statistics, ctx);
        },
        _ => {},
    }
}

fn check_for_statistics_completion(statistics: &mut Statistics, ctx: &CompilationMessageContext) {
    if compilation_has_completed(statistics) {
        notify_compiler_of_compilation_completion(ctx);
    }
}

fn circuit_break_awaited_units_if_required(statistics: &mut Statistics, type_repository: &CompilationActorHandle) {
    if !all_files_parsed(statistics) {
        return;
    }

    if compilation_has_completed(statistics) {
        return;
    }

    if number_of_awaited_units_in_pipe(statistics) == number_of_units_in_pipe(statistics) {
        circuit_break_awaited_units(type_repository);
    }
}

fn circuit_break_awaited_units(type_repository: &CompilationActorHandle) {
    circuit_break_all_type_requests(type_repository, types_not_found_type_request_circuit_break_reason());
}

fn add_file_to_statistics(statistics: &mut Statistics, filename: String) {
    statistics.files_in_pipe.insert(filename.clone(), filename);
}

fn add_awaited_unit_to_statistics(statistics: &mut Statistics, awaited_unit_id: CompilationUnitId) {
    statistics.awaited_units_in_pipe.insert(awaited_unit_id, awaited_unit_id);
}

fn remove_awaited_unit_from_statistics(statistics: &mut Statistics, awaited_unit_id: &CompilationUnitId) {
    statistics.awaited_units_in_pipe.remove(awaited_unit_id);
}

fn remove_file_from_statistics(statistics: &mut Statistics, filename: &str) {
    statistics.files_in_pipe.remove(filename);
}

fn add_unit_to_statistics(statistics: &mut Statistics, id: CompilationUnitId) {
    statistics.units_in_pipe.insert(id, id);
}

fn remove_unit_from_statistics(statistics: &mut Statistics, id: &CompilationUnitId) {
    statistics.units_in_pipe.remove(id);
}

fn all_files_parsed(statistics: &mut Statistics) -> bool {
    statistics.files_in_pipe.is_empty()
}

fn compilation_has_completed(statistics: &Statistics) -> bool {
    statistics.units_in_pipe.is_empty()
}

fn number_of_units_in_pipe(statistics: &Statistics) -> usize {
    statistics.units_in_pipe.len()
}

fn number_of_awaited_units_in_pipe(statistics: &Statistics) -> usize {
    statistics.awaited_units_in_pipe.len()
}

fn notify_compiler_of_compilation_completion(ctx: &CompilationMessageContext) {
    send_message_to_actor(&create_self_handle(ctx), create_compilation_complete_event());
}

fn log_start_compilation_phase(phase: &CompilationPhase) {
    debug!("starting {:?} compilation phase", phase);
}

fn log_end_compilation_phase(phase: &CompilationPhase) {
    debug!("ending {:?} compilation phase", phase);
}

fn log_statistics(statistics: &mut Statistics) {
    debug!("unit requsted list is now {:?}", &statistics.units_in_pipe.keys());
}