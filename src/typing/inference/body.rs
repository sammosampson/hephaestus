
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
        type_id: &mut ResolvableType
    ) {
        visit_procedure_call_return_first_return_type(self.ctx, self.type_repository, args, name, type_id);
    }

    fn visit_assignment(&mut self, _name: &mut String, value: &mut AbstractSyntaxNode, type_id: &mut ResolvableType) {
        let mut visitor = create_expression_visitor(self.ctx, self.type_repository);
        apply_visitor_to_ast_expression(value, &mut visitor);
        *type_id = resolved_resolvable_type(visitor.resolved_type.clone());
    }
}

fn visit_procedure_call_return_first_return_type(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    args: &mut AbstractSyntaxChildNodes,
    name: &mut String, 
    type_id: &mut ResolvableType
) -> ResolvedTypeIds {
    let mut visitor = create_procedure_call_visitor(ctx, type_repository);
    apply_visitor_to_ast_procedure_call(args, &mut visitor);

    let resolved_type = find_type(
        create_find_type_criteria(name.to_string(), visitor.resolved_call_arg_types),
        ctx,
        type_repository
    );

    *type_id = resolved_resolvable_type(resolved_type.id);

    if let TypeItem::ProcedureDefinition { return_types, .. } = resolved_type.item {
        return return_types;
    }

    vec!()
}


struct ProcedureCallInferenceVisitor <'a> { 
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    resolved_call_arg_types: ResolvedTypeIds
}

fn create_procedure_call_visitor<'a>(
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle
) -> ProcedureCallInferenceVisitor::<'a> {
    ProcedureCallInferenceVisitor { 
        ctx,
        type_repository, 
        resolved_call_arg_types: vec!() 
    }
}

impl <'a> AbstractSyntaxProcedureCallNodeVisitor for ProcedureCallInferenceVisitor<'a> {
    fn visit_argument(&mut self, expr: &mut AbstractSyntaxNode, arg_type: &mut ResolvableType) {
        let mut visitor = create_expression_visitor(self.ctx, self.type_repository);
        apply_visitor_to_ast_expression(expr, &mut visitor);
        *arg_type = resolved_resolvable_type(visitor.resolved_type.clone());
        self.resolved_call_arg_types.push(visitor.resolved_type);
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
            Literal::Int(_value) => self.resolved_type = built_in_type_resolved_type_id(int_32_built_in_type()),
            _ => todo!(),
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
            args,
            name,
            type_id
        );
        
        if resolved_types.len() > 0 {
            self.resolved_type = resolved_types.first().unwrap().clone();
        }
    }
}