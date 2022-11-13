use std::collections::HashMap;
use log::*;

use crate::{
    compilation::*,
    parsing::*,
    acting::*
};

pub type Statistics = HashMap<CompilationUnitId, CompilationUnitId>;

pub fn create_statistics() -> Statistics {
    HashMap::default()
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

pub fn end_compilation_phase(statistics: &mut Statistics, phase: CompilationPhase, ctx: &CompilationMessageContext) {
    log_end_compilation_phase(&phase);
    perform_end_compilation_phase(statistics, phase, ctx);
    log_statistics(statistics);
}

fn perform_start_compilation_phase(statistics: &mut Statistics, phase: CompilationPhase) {
    match phase {
        CompilationPhase::Typing(id) => 
            add_unit_to_statistics(statistics, id),
        _ => {},
    }
}

fn perform_end_compilation_phase(statistics: &mut Statistics, phase: CompilationPhase, ctx: &CompilationMessageContext) {
    match phase {
        CompilationPhase::BackendBuild(id) => {
            remove_unit_from_statistics(statistics, &id);
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

fn add_unit_to_statistics(lookup: &mut Statistics, id: CompilationUnitId) {
    lookup.insert(id, id);
}

fn remove_unit_from_statistics(lookup: &mut Statistics, id: &CompilationUnitId) {
    lookup.remove(id);
}

fn compilation_has_completed(lookup: &Statistics) -> bool {
    lookup.is_empty()
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
    debug!("unit requsted list is now {:?}", &statistics.keys());
}