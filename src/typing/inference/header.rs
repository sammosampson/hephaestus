use crate::parsing::*;
use crate::typing::*;

pub struct ProcedureHeaderInferenceVisitor {
    pub arg_types: RuntimeTypePointers,
    pub return_types: RuntimeTypePointers
}

pub fn create_procedure_header_visitor() -> ProcedureHeaderInferenceVisitor {
    ProcedureHeaderInferenceVisitor { arg_types: vec!(), return_types: vec!() }
}

impl AbstractSyntaxProcedureHeaderNodeVisitor for ProcedureHeaderInferenceVisitor {
    fn visit_argument_declaration(&mut self, _name: &mut String, arg_type: &mut ResolvableType) {
        try_parse_resolved_runtime_type_pointer(arg_type, &mut self.arg_types);
    }

    fn visit_return_type_declaration(&mut self, return_type: &mut ResolvableType) {
        try_parse_resolved_runtime_type_pointer(return_type, &mut self.return_types);
    }
}

fn try_parse_resolved_runtime_type_pointer(arg_type: &ResolvableType, type_ids: &mut RuntimeTypePointers) {
    if let Some(resolved_runtime_type_pointer) = try_get_resolved_runtime_type_pointer(arg_type) {
        type_ids.push(resolved_runtime_type_pointer);
    }
}
