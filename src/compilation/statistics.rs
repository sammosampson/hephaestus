use std::collections::HashMap;
use log::*;

use crate::{
    compilation::*,
    parsing::*,
    acting::*
};

pub type Statistics = HashMap<CompilationPhase, CompilationPhase>;

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
    register_unit_with_statistics(statistics, phase);
    log_statistics(statistics);
}

pub fn end_compilation_phase(statistics: &mut Statistics, phase: CompilationPhase, ctx: &CompilationMessageContext) {
    log_end_compilation_phase(&phase);
    remove_unit_from_statistics(statistics, &phase);
    check_for_statistics_completion(statistics, ctx);
    log_statistics(statistics);
}

pub fn check_for_statistics_completion(statistics: &mut Statistics, ctx: &CompilationMessageContext) {
    if compilation_has_completed(statistics) {
        notify_compiler_of_compilation_completion(ctx);
    }
}

fn register_unit_with_statistics(lookup: &mut Statistics, phase: CompilationPhase) {
    lookup.insert(phase.clone(), phase);
}

fn remove_unit_from_statistics(lookup: &mut Statistics, phase: &CompilationPhase) {
    lookup.remove(phase);
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