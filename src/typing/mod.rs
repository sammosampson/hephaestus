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