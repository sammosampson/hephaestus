mod repository;
mod inference;
pub use repository::*;
pub use inference::*;

use crate::{
    parsing::*,
    threading::*
};

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvableType {
    Resolved(RuntimeTypePointer),
    UnresolvedNamed(String),
    Unresolved
}

pub fn unresolved_resolvable_type() -> ResolvableType {
    ResolvableType::Unresolved
}

pub fn unresolved_named_resolvable_type(name: String) -> ResolvableType {
    ResolvableType::UnresolvedNamed(name)
}

pub fn resolved_resolvable_type(type_pointer: RuntimeTypePointer) -> ResolvableType {
    ResolvableType::Resolved(type_pointer)
}

pub fn try_get_resolved_runtime_type_pointer(resolvable_type: &ResolvableType) -> Option<RuntimeTypePointer> {
    if let ResolvableType::Resolved(pointer) = resolvable_type {
       return Some(pointer.clone());
    }
    None
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub enum RuntimeTypeId {
    BuiltInType(BuiltInType),
    UserDefined(CompilationUnitId)
}

pub fn built_in_type_runtime_type_id(built_in_type: BuiltInType) -> RuntimeTypeId {
    RuntimeTypeId::BuiltInType(built_in_type)
}

pub fn user_defined_runtime_type_id(unit_id: CompilationUnitId) -> RuntimeTypeId {
    RuntimeTypeId::UserDefined(unit_id)
}

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


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct RuntimeType {
    pub id: RuntimeTypeId,
    pub name: String,
    pub item: RuntimeTypeItem,
    pub size: TypeSize
}

pub type RuntimeTypePointer = Shareable<RuntimeType>;
pub type RuntimeTypePointers = Vec<RuntimeTypePointer>;

pub fn create_type(id: RuntimeTypeId, name: String, item: RuntimeTypeItem, size: TypeSize) -> RuntimeType {
    RuntimeType {
        id: id,
        name,
        item,
        size
    }
}

impl From<BuiltInType> for RuntimeType {
    fn from(from: BuiltInType) -> Self {
        match from {
            BuiltInType::Int32 => int_32_runtime_type(),
            BuiltInType::Float32 => float_32_runtime_type(),
            BuiltInType::Void => void_runtime_type(),
        }
    }
}

pub fn int_32_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(int_32_built_in_type()),
        "i32".to_string(),
        integer_type_item(),
        resolved_type_size(4)
    )
}

pub fn float_32_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(float_32_built_in_type()),
        "f32".to_string(),
        float_type_item(),
        resolved_type_size(4)
    )
}

pub fn void_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(void_built_in_type()),
        "void".to_string(),
        void_type_item(),
        not_required_type_size()
    )
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum RuntimeTypeItem {
    None,
    ProcedureDefinition { arg_types: RuntimeTypePointers, return_types: RuntimeTypePointers },
    Int,
    Float,
    Void
}

impl Default for RuntimeTypeItem {
    fn default() -> Self {
        RuntimeTypeItem::None
    }
}

pub fn procedure_definition_type_item(arg_types: RuntimeTypePointers, return_types: RuntimeTypePointers) -> RuntimeTypeItem {
    RuntimeTypeItem::ProcedureDefinition { arg_types, return_types }
}

fn integer_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Int
}

fn float_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Float
}

fn void_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Void
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum TypeSize {
    NotRequired,
    Resolved { size_in_bytes: usize },
}

pub fn resolved_type_size(size_in_bytes: usize) -> TypeSize {
    TypeSize::Resolved { size_in_bytes }
}

pub fn not_required_type_size() -> TypeSize {
    TypeSize::NotRequired
}