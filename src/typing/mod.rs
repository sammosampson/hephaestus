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

pub fn unresolved_resolvable_type() -> ResolvableType {
    ResolvableType::Unresolved
}

pub fn unresolved_named_resolvable_type(name: String) -> ResolvableType {
    ResolvableType::UnresolvedNamed(name)
}
pub fn resolved_resolvable_type(type_id: ResolvedTypeId) -> ResolvableType {
    ResolvableType::Resolved(type_id)
}

pub fn try_get_built_in_type_from_resolved_resolvable_type(resolvable_type: &ResolvableType) -> Option<BuiltInType> {
    if let ResolvableType::Resolved(ResolvedTypeId::BuiltInType(built_in_type)) = resolvable_type {
       return Some(*built_in_type);
    }
    None
}

pub type ResolvedTypes = Vec<ResolvedType>;

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub enum ResolvedTypeId {
    NotResolved,
    BuiltInType(BuiltInType),
    UserDefined(CompilationUnitId)
}

pub fn built_in_type_resolved_type_id(built_in_type: BuiltInType) -> ResolvedTypeId {
    ResolvedTypeId::BuiltInType(built_in_type)
}

pub fn user_defined_resolved_type_id(unit_id: CompilationUnitId) -> ResolvedTypeId {
    ResolvedTypeId::UserDefined(unit_id)
}

pub fn not_resolved_type_id() -> ResolvedTypeId {
    ResolvedTypeId::NotResolved
}

pub type ResolvedTypeIds = Vec<ResolvedTypeId>;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum BuiltInType {
    Int32,
    Float32,
    Void
}

pub fn void_built_in_type() -> BuiltInType {
    BuiltInType::Void
}

pub fn int_32_built_in_type() -> BuiltInType {
    BuiltInType::Int32
}

pub fn float_32_built_in_type() -> BuiltInType {
    BuiltInType::Float32
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
        id: user_defined_resolved_type_id(id),
        name,
        item,
        size: unresolved_type_size()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeItem {
    None,
    ProcedureDefinition { arg_types: ResolvedTypeIds, return_types: ResolvedTypeIds },
}

impl Default for TypeItem {
    fn default() -> Self {
        TypeItem::None
    }
}

pub fn procedure_definition_type_item(arg_types: ResolvedTypeIds, return_types: ResolvedTypeIds) -> TypeItem {
    TypeItem::ProcedureDefinition { arg_types, return_types }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeSize {
    Unresolved,
}

pub fn unresolved_type_size() -> TypeSize {
    TypeSize::Unresolved
}