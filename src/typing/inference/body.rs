
use crate::parsing::*;
use crate::compilation::*;
use crate::typing::*;

pub struct ProcedureBodyInferenceVisitor<'a> { 
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle
}

pub fn create_procedure_body_visitor<'a>(
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle
) -> ProcedureBodyInferenceVisitor<'a> {
    ProcedureBodyInferenceVisitor::<'a> { ctx, type_repository }
}

impl <'a> AbstractSyntaxProcedureBodyNodeVisitor for ProcedureBodyInferenceVisitor<'a> {
    fn visit_procedure_call(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        type_id: &mut ResolvableType,
        return_type_ids: &mut ResolvableTypes
    ) {
        visit_procedure_call(self.ctx, self.type_repository, args, name, type_id, return_type_ids);
    }

    fn visit_assignment(&mut self, _name: &mut String, _value: &mut AbstractSyntaxNode) {
    }
}

fn visit_procedure_call(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    args: &mut AbstractSyntaxChildNodes,
    name: &mut String, 
    type_id: &mut ResolvableType, 
    _return_type_ids: &mut ResolvableTypes
) {
    let mut visitor = create_procedure_call_visitor(ctx, type_repository);
    apply_visitor_to_ast_procedure_call(args, &mut visitor);

    let resolved_type = find_type(
        create_find_type_criteria(name.to_string(), visitor.resolved_types),
        ctx,
        type_repository
    );

    *type_id = resolved_resolvable_type(resolved_type.id);

    if let Some(found_type_return_types) = try_get_procedure_definition_type_item(resolved_type.item) {
        for _return_type in found_type_return_types {

        }
        //*return_type_ids = found_type_return_types;
    }
}


struct ProcedureCallInferenceVisitor <'a> { 
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    resolved_types: ResolvedTypeIds
}

fn create_procedure_call_visitor<'a>(
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle
) -> ProcedureCallInferenceVisitor::<'a> {
    ProcedureCallInferenceVisitor { 
        ctx,
        type_repository, 
        resolved_types: vec!() 
    }
}

impl <'a> AbstractSyntaxProcedureCallNodeVisitor for ProcedureCallInferenceVisitor<'a> {
    fn visit_argument(&mut self, expr: &mut AbstractSyntaxNode, arg_type: &mut ResolvableType) {
        let mut visitor = create_expression_visitor(self.ctx, self.type_repository);
        apply_visitor_to_ast_expression(expr, &mut visitor);
        *arg_type = ResolvableType::Resolved(visitor.resolved_type.clone());
        self.resolved_types.push(visitor.resolved_type);
    }
}

pub struct ExpressionInferenceVisitor<'a> { 
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    resolved_type: ResolvedTypeId
}

pub fn create_expression_visitor<'a>(
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle
) -> ExpressionInferenceVisitor<'a> {
    ExpressionInferenceVisitor::<'a> {
        ctx,
        type_repository, 
        resolved_type: not_resolved_type_id()
    }
}

impl <'a> AbstractSyntaxExpressionNodeVisitor for ExpressionInferenceVisitor<'a> {
    fn visit_literal(&mut self, literal: &mut Literal) {
        match literal {
            Literal::Int(_value) => self.resolved_type = ResolvedTypeId::BuiltInType(BuiltInType::Int32),
            _ => todo!(),
        }
    }

    fn visit_procedure_call(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        type_id: &mut ResolvableType,
        return_type_ids: &mut ResolvableTypes
    ) {
        visit_procedure_call(self.ctx, self.type_repository, args, name, type_id, return_type_ids);
    }
}