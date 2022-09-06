mod repository;
mod inference;
pub use repository::*;
pub use inference::*;

use crate::parsing::*;

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvableType {
    Resolved(ResolvedTypeId),
    UnresolvedNamed(String),
    Unresolved
}

pub type ResolvedTypes = Vec<ResolvedType>;

#[derive(PartialEq, Debug, Clone, Hash)]
pub enum ResolvedTypeId {
    NotResolved,
    BuiltInType(BuiltInType),
    UserDefined(CompilationUnitId)
}

pub type ResolvedTypeIds = Vec<ResolvedTypeId>;

#[derive(PartialEq, Debug, Clone, Copy, Hash)]
pub enum BuiltInType {
    Int32,
    Float32,
    Void
}

pub fn create_built_in_type_id(built_in_type: &BuiltInType) -> ResolvedTypeId {
    ResolvedTypeId::BuiltInType(*built_in_type)
}

#[derive(PartialEq, Debug, Clone)]
pub struct ResolvedType {
    pub id: ResolvedTypeId,
    pub name: String,
    pub item: TypeItem,
    pub size: TypeSize
}

pub fn create_type(id: CompilationUnitId, name: String, item: TypeItem) -> ResolvedType {
    ResolvedType {
        id: ResolvedTypeId::UserDefined(id),
        name,
        item,
        size: TypeSize::Unresolved
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeItem {
    ProcedureDefinition { arg_types: ResolvedTypeIds, return_types: ResolvedTypeIds },
}

pub fn create_procedure_defnition_type_item(arg_types: ResolvedTypeIds, return_types: ResolvedTypeIds) -> TypeItem {
    TypeItem::ProcedureDefinition { arg_types, return_types }
}


#[derive(PartialEq, Debug, Clone)]
pub enum TypeSize {
    Unresolved,
}