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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CompilationPhase{
    Parsing,
    Typing,
    Sizing,
    ByteCodeCreation,
    BackendBuild
}

pub fn parsing_compilation_phase() -> CompilationPhase {
    CompilationPhase::Parsing
}

pub fn typing_compilation_phase() -> CompilationPhase {
    CompilationPhase::Typing
}

pub fn sizing_compilation_phase() -> CompilationPhase {
    CompilationPhase::Sizing
}

pub fn byte_code_creation_compilation_phase() -> CompilationPhase {
    CompilationPhase::ByteCodeCreation
}

pub fn backend_build_compilation_phase() -> CompilationPhase {
    CompilationPhase::BackendBuild
}

pub fn register_units_with_statistics(
    statistics: &mut Statistics,
    units: &CompilationUnits
) {
    for unit in units {
        log_end_compilation_phase(parsing_compilation_phase(), unit.id);
        register_unit_with_statistics(statistics, unit.id);
    }
    
    log_statistics(statistics);
}

pub fn start_compilation_phase(_statistics: &mut Statistics, phase: CompilationPhase, id: CompilationUnitId) {
    log_start_compilation_phase(phase, id);
}


pub fn end_compilation_phase(statistics: &mut Statistics, phase: CompilationPhase, id: CompilationUnitId, ctx: &CompilationMessageContext) {
    log_end_compilation_phase(phase, id);
    if is_final_phase(phase) {
        remove_unit_from_statistics_and_check_for_completion(statistics, id, ctx);
    }
}

fn is_final_phase(phase: CompilationPhase) -> bool {
    phase == CompilationPhase::BackendBuild
}

fn remove_unit_from_statistics_and_check_for_completion(statistics: &mut Statistics, id: CompilationUnitId, ctx: &CompilationMessageContext) {
    remove_unit_from_statistics(statistics, &id);
    log_statistics(statistics);
    check_for_statistics_completion(statistics, ctx);
}

pub fn check_for_statistics_completion(statistics: &mut Statistics, ctx: &CompilationMessageContext) {
    if compilation_has_completed(statistics) {
        notify_compiler_of_compilation_completion(ctx);
    }
}

fn register_unit_with_statistics(lookup: &mut Statistics, id: CompilationUnitId) {
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

fn log_start_compilation_phase(phase: CompilationPhase, id: CompilationUnitId) {
    debug!("starting {:?} compilation phase for {:?}", phase, id);
}

fn log_end_compilation_phase(phase: CompilationPhase, id: CompilationUnitId) {
    debug!("ending {:?} compilation phase for {:?}", phase, id);
}

fn log_statistics(statistics: &mut Statistics) {
    debug!("unit requsted list is now {:?}", &statistics.keys());
}