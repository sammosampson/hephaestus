use std::collections::HashMap;
use log::*;

use crate::{
    compilation::*,
    parsing::*,
    backends::*,
    file_system::*
};

pub type CompilationUnitsRequestedList = HashMap<CompilationUnitId, CompilationUnitId>;

pub fn register_units_with_statistics<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    units: &Vec<CompilationUnit>
) {
    for unit in units {
        register_compilation_requested(&mut compiler.compilation_units_requested_list, unit.id);
    }
    
    debug!("unit requsted list is now {:?}", &compiler.compilation_units_requested_list.keys());
}

pub fn create_compilation_units_requested_list() -> HashMap<CompilationUnitId, CompilationUnitId> {
    HashMap::default()
}

pub fn register_compilation_requested(lookup: &mut CompilationUnitsRequestedList, id: CompilationUnitId) {
    lookup.insert(id, id);
}

pub fn remove_unit_from_compilation_requested_list(lookup: &mut CompilationUnitsRequestedList, id: &CompilationUnitId) {
    lookup.remove(id);
}

pub fn compilation_requested_list_is_empty(lookup: &CompilationUnitsRequestedList) -> bool {
    lookup.is_empty()
}