use crate::parsing::*;
use crate::typing::*;

pub struct ProcedureHeaderInferenceVisitor {
    pub arg_types: ResolvedTypeIds,
    pub return_types: ResolvedTypeIds
}

pub fn create_procedure_header_visitor() -> ProcedureHeaderInferenceVisitor {
    ProcedureHeaderInferenceVisitor { arg_types: vec!(), return_types: vec!() }
}

impl AbstractSyntaxProcedureHeaderNodeVisitor for ProcedureHeaderInferenceVisitor {
    fn visit_argument_declaration(&mut self, _name: &mut String, arg_type: &mut ResolvableType) {
        parse_built_in_arg_type(arg_type, &mut self.arg_types);
    }

    fn visit_return_type_declaration(&mut self, return_type: &mut ResolvableType) {
        parse_built_in_arg_type(return_type, &mut self.return_types);
    }
}

fn parse_built_in_arg_type(arg_type: &ResolvableType, type_ids: &mut ResolvedTypeIds) {
    if let ResolvableType::Resolved(ResolvedTypeId::BuiltInType(built_in_type)) = arg_type {
        type_ids.push(create_built_in_type_id(built_in_type));
    }
}
