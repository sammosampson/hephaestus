mod repository;
pub use repository::*;

use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;

pub struct TypingActor;

impl Actor<CompilationMessage> for TypingActor {
    fn receive(&self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::PerformTyping { unit, type_repository, compiler} => 
                handle_perform_typing(unit, ctx, type_repository, compiler),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_perform_typing(
    mut unit: CompilationUnit, 
    ctx: &CompilationMessageContext,
    type_repository: CompilationActorHandle, 
    compiler: CompilationActorHandle
) -> AfterReceiveAction {
    let resolved_types = perform_typing(&mut unit, | criteria| find_type(criteria, ctx, &type_repository));
    send_message_to_actor(&compiler, create_unit_typed_event(resolved_types, unit));
    shutdown_after_receive()
}

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvableType {
    Resolved(ResolvedTypeId),
    UnresolvedNamed(String),
    Unresolved
}

pub type ResolvedTypeIds = Vec<ResolvedTypeId>;

#[derive(PartialEq, Debug, Clone, Hash)]
pub enum ResolvedTypeId {
    NotResolved,
    BuiltInType(BuiltInType),
    UserDefined(CompilationUnitId)
}

#[derive(PartialEq, Debug, Clone, Copy, Hash)]
pub enum BuiltInType {
    Int32,
    Float32,
    Void
}

#[derive(PartialEq, Debug, Clone)]
pub struct ResolvedType {
    pub id: ResolvedTypeId,
    pub name: String,
    pub item: TypeItem,
    pub size: TypeSize
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeItem {
    ProcedureDefinition { arg_types: ResolvedTypeIds, return_types: ResolvedTypeIds },
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeSize {
    Unresolved,
}


pub type ResolvedTypes = Vec<ResolvedType>;

pub fn perform_typing<FT: Fn(FindTypeCriteria) -> ResolvedTypeId>(unit: &mut CompilationUnit, find_type: FT) -> ResolvedTypes {
    let mut types = vec!();
    
    match unit.tree.item_ref() {
        AbstractSyntaxNodeItem::ProcedureHeader { name, args: arguments, return_types, .. } => {
            types.push(
                create_type(
                    unit.id,
                    name.clone(), 
                    TypeItem::ProcedureDefinition { arg_types: parse_arg_types(arguments), return_types: parse_return_types(return_types) }
                )
            );
        },
        item => println!("unimplemented ast item for typing {:?}", item)
    };

    types
}

fn parse_arg_types(args: &AbstractSyntaxChildNodes) -> ResolvedTypeIds {
    let mut ids = vec!();

    for arg in args {
        parse_arg_type(arg, &mut ids)
    }

    ids
}

fn parse_arg_type(arg: &AbstractSyntaxNode, type_ids: &mut ResolvedTypeIds) {
    if let AbstractSyntaxNodeItem::ArgumentDeclaration { arg_type, .. } = arg.item_ref() {
        if let ResolvableType::Resolved(ResolvedTypeId::BuiltInType(built_in_type)) = arg_type {
            type_ids.push(ResolvedTypeId::BuiltInType(*built_in_type));
        }
    }
}

fn parse_return_types(args: &AbstractSyntaxChildNodes) -> ResolvedTypeIds {
    let mut ids = vec!();

    for arg in args {
        parse_return_type(arg, &mut ids)
    }

    ids
}

fn parse_return_type(arg: &AbstractSyntaxNode, type_ids: &mut ResolvedTypeIds) {
    if let AbstractSyntaxNodeItem::Type(return_type) = arg.item_ref() {
        if let ResolvableType::Resolved(ResolvedTypeId::BuiltInType(built_in_type)) = return_type {
            type_ids.push(ResolvedTypeId::BuiltInType(*built_in_type));
        }
    }
}

fn create_type(id: CompilationUnitId, name: String, item: TypeItem) -> ResolvedType {
    ResolvedType {
        id: ResolvedTypeId::UserDefined(id),
        name,
        item,
        size: TypeSize::Unresolved
    }
}