
use std::collections::HashMap;

use crate::parsing::*;
use crate::compilation::*;
use crate::threading::create_shareable;
use crate::typing::*;

pub type LocalTypeMap = HashMap<String, RuntimeTypePointer>;
pub type LocalTypes = RuntimeTypePointers;

fn add_local_type_to_map(map: &mut LocalTypeMap, identifier: String, resolved_type: RuntimeTypePointer) {
    map.insert(identifier, resolved_type);
}

fn get_type_for_local_identifier<'a>(map: &'a LocalTypeMap, identifier: &str) -> Option<&'a RuntimeTypePointer> {
    map.get(identifier)
}

pub fn create_local_type_map() -> LocalTypeMap {
    LocalTypeMap::default()
}

pub struct ProcedureBodyInferenceVisitor<'a> { 
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    local_type_map: LocalTypeMap,
    local_return_types: LocalTypes
}

pub fn create_procedure_body_visitor<'a>(
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle
) -> ProcedureBodyInferenceVisitor<'a> {
    ProcedureBodyInferenceVisitor::<'a> {
        ctx,
        type_repository,
        local_type_map: create_local_type_map(),
        local_return_types: vec!()
    }
}

impl <'a> AbstractSyntaxProcedureBodyNodeVisitor for ProcedureBodyInferenceVisitor<'a> {
    fn visit_argument_declaration(&mut self, name: &mut String, arg_type: &mut ResolvableType) {
        if let Some(resolved_type) = try_get_resolved_runtime_type_pointer(arg_type) {
            add_local_type_to_map(
                &mut self.local_type_map,
                name.clone(),
                resolved_type
            );    
        }
    }

    fn visit_return_type_declaration(&mut self, return_type: &mut ResolvableType) {      
        if let Some(resolved_type) = try_get_resolved_runtime_type_pointer(return_type) {
            self.local_return_types.push(resolved_type);
        }
    }

    fn visit_return_statement(&mut self, args: &mut AbstractSyntaxChildNodes) {
        let mut visitor = create_args_visitor(
            self.ctx,
            self.type_repository, 
            &self.local_type_map
        );

        apply_visitor_to_ast_args(args, &mut visitor);
        
        if visitor.resolved_arg_types != self.local_return_types {
            todo!("handle differences between what is returned and procedure header return types")
        }
    }

    fn visit_procedure_call(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        type_id: &mut ResolvableType
    ) {   
        visit_procedure_call_return_first_return_type(
            self.ctx,
            self.type_repository,
            &self.local_type_map,
            args,
            name,
            type_id
        );
    }

    fn visit_assignment(&mut self, name: &mut String, value: &mut AbstractSyntaxNode, resolvable_type: &mut ResolvableType) {
        let mut visitor = create_expression_visitor(self.ctx, self.type_repository, &self.local_type_map);
        apply_visitor_to_ast_expression(value, &mut visitor);
        
        if let Some(resolved_type) = visitor.resolved_type {
            *resolvable_type = resolved_resolvable_type(resolved_type.clone());
            add_local_type_to_map(&mut self.local_type_map, name.clone(), resolved_type);
        }
    }
}

fn visit_procedure_call_return_first_return_type(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &LocalTypeMap,
    args: &mut AbstractSyntaxChildNodes,
    name: &mut String, 
    type_id: &mut ResolvableType
) -> RuntimeTypePointers {  
    let mut visitor = create_args_visitor(ctx, type_repository, local_type_map);
    apply_visitor_to_ast_args(args, &mut visitor);

    let resolved_type = find_type(
        create_find_type_criteria(name.to_string(), visitor.resolved_arg_types),
        ctx,
        type_repository
    );

    *type_id = resolved_resolvable_type(resolved_type.clone());
    
    if let RuntimeTypeItem::ProcedureDefinition { return_types, .. } = &resolved_type.item {
        return return_types.clone();
    }

    vec!()
}


struct ArgsInferenceVisitor <'a> { 
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    local_type_map: &'a LocalTypeMap,
    resolved_arg_types: RuntimeTypePointers
}

fn create_args_visitor<'a>(
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    local_type_map: &'a LocalTypeMap,
) -> ArgsInferenceVisitor::<'a> {
    ArgsInferenceVisitor { 
        ctx,
        type_repository, 
        local_type_map,
        resolved_arg_types: vec!() 
    }
}

impl <'a> AbstractSyntaxArgumentsNodeVisitor for ArgsInferenceVisitor<'a> {
    fn visit_argument(&mut self, expr: &mut AbstractSyntaxNode, arg_type: &mut ResolvableType) {
        let mut visitor = create_expression_visitor(
            self.ctx,
            self.type_repository,
            self.local_type_map
        );
        
        apply_visitor_to_ast_expression(expr, &mut visitor);

        if let Some(resolved_type) = visitor.resolved_type {
            *arg_type = resolved_resolvable_type(resolved_type.clone());
            self.resolved_arg_types.push(resolved_type);
        }
    }
}

pub struct ExpressionInferenceVisitor<'a> { 
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    local_type_map: &'a LocalTypeMap,
    resolved_type: Option<RuntimeTypePointer>
}

pub fn create_expression_visitor<'a>(
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    local_type_map: &'a LocalTypeMap
) -> ExpressionInferenceVisitor<'a> {
    ExpressionInferenceVisitor::<'a> {
        ctx,
        type_repository,
        local_type_map, 
        resolved_type: None
    }
}

impl <'a> AbstractSyntaxExpressionNodeVisitor for ExpressionInferenceVisitor<'a> {
    fn visit_literal(&mut self, literal: &mut Literal) {
        match literal {
            Literal::UnsignedInt(_value) => self.resolved_type = Some(create_shareable(signed_int_32_runtime_type())),
            Literal::Float(_value) => self.resolved_type = Some(create_shareable(float_32_runtime_type())),
            Literal::String(_value) => self.resolved_type = Some(create_shareable(string_runtime_type()))
        }
    }

    fn visit_identifier(&mut self, name: &mut String) {
        if let Some(local_identifier_type) = get_type_for_local_identifier(&self.local_type_map, &name) {
            self.resolved_type = Some(local_identifier_type.clone());
        } else {
            todo!("look global scope for identifiers and other external places")
        }
    }

    fn visit_expression(
        &mut self,
        _op: &mut AbstractSyntaxNode,
        lhs: &mut AbstractSyntaxNode,
        rhs: &mut AbstractSyntaxNode,
        type_id: &mut ResolvableType
    ) {
        let mut visitor = create_expression_visitor(self.ctx, self.type_repository, self.local_type_map);
        
        apply_visitor_to_ast_expression(lhs, &mut visitor);
        let lhs_resolved_type = visitor.resolved_type.clone();
        
        apply_visitor_to_ast_expression(rhs, &mut visitor);
        let rhs_resolved_type = visitor.resolved_type.clone();
        
        if lhs_resolved_type != rhs_resolved_type {
            todo!("deal with different types on either side of expression")
        }

        if let Some(resolved_type) = lhs_resolved_type {
            *type_id = resolved_resolvable_type(resolved_type.clone());
            self.resolved_type = Some(resolved_type);
        }
    }

    fn visit_procedure_call(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        type_id: &mut ResolvableType
    ) {
        let resolved_types = visit_procedure_call_return_first_return_type(
            self.ctx,
            self.type_repository,
            self.local_type_map,
            args,
            name,
            type_id
        );
        
        if resolved_types.len() > 0 {
            self.resolved_type = Some(resolved_types.first().unwrap().clone());
        }
    }

    fn visit_foreign_system_library(&mut self, _library: &mut AbstractSyntaxNode) {
    }
}