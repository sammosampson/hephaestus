use std::collections::HashMap;

use crate::{
    parsing::*
};

pub type CompilationUnitsRequestedList = HashMap<CompilationUnitId, CompilationUnitId>;

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