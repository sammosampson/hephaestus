mod header;
mod body;
pub use header::*;
pub use body::*;

use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;
use crate::typing::*;

pub struct TypingActor;

pub fn create_typing_actor() -> TypingActor {
    TypingActor
}

impl Actor<CompilationMessage> for TypingActor {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::PerformTyping { unit, type_repository, compiler} => 
                handle_perform_typing(unit, ctx, &type_repository, compiler),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_perform_typing(
    mut unit: CompilationUnit, 
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle, 
    compiler: CompilationActorHandle
) -> AfterReceiveAction {
    let resolved_types = perform_typing(ctx, type_repository, &mut unit);
    send_message_to_actor(&compiler, create_unit_typed_event(resolved_types, unit));
    
    shutdown_after_receive()
}

pub fn perform_typing(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    unit: &mut CompilationUnit
) -> RuntimeTypePointers {
    let mut visitor = create_root_visitor(ctx, type_repository, unit.id);    
    apply_visitor_to_ast_root(&mut unit.tree, &mut visitor);

    visitor.resolved_types
}

pub struct RootInferenceVisitor<'a> { 
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    unit_id: CompilationUnitId,
    resolved_types: RuntimeTypePointers
}

fn create_root_visitor<'a>(
    ctx: &'a CompilationMessageContext,
    type_repository: &'a CompilationActorHandle,
    unit_id: CompilationUnitId
) -> RootInferenceVisitor<'a> {
    RootInferenceVisitor {
        ctx,
        type_repository,
        unit_id,
        resolved_types: vec!()
    }
}

impl <'a> AbstractSyntaxRootNodeVisitor for RootInferenceVisitor<'a> {
    fn visit_run(&mut self, expr: &mut AbstractSyntaxNode) {
        let local_type_map = create_local_type_map();
        let mut visitor = create_expression_visitor(self.ctx, self.type_repository, &local_type_map);
        apply_visitor_to_ast_expression(expr, &mut visitor);
        dbg!(expr);
    }

    fn visit_procedure_header(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        return_types: &mut AbstractSyntaxChildNodes,
        _body: &mut ProcedureBodyReference
    ) {    
        let mut visitor = create_procedure_header_visitor();
        apply_visitor_to_ast_procedure_header(args, return_types, &mut visitor);        

        let resolved_type = create_type(
            user_defined_runtime_type_id(self.unit_id),
            name.clone(),
            procedure_definition_type_item(visitor.arg_types, visitor.return_types),
            not_required_type_size()
        );
        
        self.resolved_types.push(create_shareable(resolved_type));    
    }

    fn visit_procedure_body(
        &mut self,
        _name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        return_types: &mut AbstractSyntaxChildNodes,
        statements: &mut AbstractSyntaxChildNodes
    ) {       
        let mut visitor = create_procedure_body_visitor(self.ctx, self.type_repository);
        apply_visitor_to_ast_procedure_body(args, return_types, statements, &mut visitor);        
    }
}
