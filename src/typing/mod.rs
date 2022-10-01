mod repository;
mod inference;

pub use repository::*;
pub use inference::*;

use std::mem::*;

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
    BuiltInType { built_in_type: BuiltInType, is_pointer: bool },
    UserDefined { unit_id: CompilationUnitId, is_pointer: bool }
}

pub fn built_in_type_runtime_type_id(built_in_type: BuiltInType) -> RuntimeTypeId {
    RuntimeTypeId::BuiltInType { built_in_type, is_pointer: false }
}

pub fn built_in_type_pointer_runtime_type_id(built_in_type: BuiltInType) -> RuntimeTypeId {
    RuntimeTypeId::BuiltInType { built_in_type, is_pointer: true }
}

pub fn user_defined_runtime_type_id(unit_id: CompilationUnitId) -> RuntimeTypeId {
    RuntimeTypeId::UserDefined { unit_id, is_pointer: false }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum BuiltInType {
    SignedInt32,
    Float32,
    String,
    Boolean,
    Void
}

pub fn void_built_in_type() -> BuiltInType {
    BuiltInType::Void
}

pub fn signed_int_32_built_in_type() -> BuiltInType {
    BuiltInType::SignedInt32
}

pub fn float_32_built_in_type() -> BuiltInType {
    BuiltInType::Float32
}

pub fn string_built_in_type() -> BuiltInType {
    BuiltInType::String
}

pub fn bool_built_in_type() -> BuiltInType {
    BuiltInType::Boolean
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

pub fn signed_int_32_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(signed_int_32_built_in_type()),
        "s32".to_string(),
        integer_type_item(),
        resolved_type_size(4)
    )
}

pub fn signed_int_32_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(signed_int_32_built_in_type()),
        signed_int_32_runtime_type()
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

pub fn float_32_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(float_32_built_in_type()),
        float_32_runtime_type()
    )
}


pub fn string_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(string_built_in_type()),
        "string".to_string(),
        string_type_item(),
        resolved_type_size(4)
    )
}

pub fn string_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(string_built_in_type()),
        string_runtime_type()
    )
}

pub fn bool_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(bool_built_in_type()),
        "bool".to_string(),
        bool_type_item(),
        resolved_type_size(1)
    )
}

pub fn bool_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(bool_built_in_type()),
        bool_runtime_type()
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

pub fn void_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(void_built_in_type()),
        void_runtime_type()
    )
}

pub fn pointer_runtime_type(id: RuntimeTypeId, to_type: RuntimeType) -> RuntimeType {
    create_type(
        id,
        format!("*{}", to_type.name).to_string(),
        pointer_type_item(Box::new(to_type)),
        resolved_type_size(size_of::<usize>())
    )
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum RuntimeTypeItem {
    None,
    ProcedureDefinition { arg_types: RuntimeTypePointers, return_types: RuntimeTypePointers },
    Pointer { to_type: Box<RuntimeType> },
    Int,
    Float,
    String,
    Bool,
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

pub fn pointer_type_item(to_type: Box<RuntimeType>) -> RuntimeTypeItem {
    RuntimeTypeItem::Pointer { to_type }
}

fn integer_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Int
}

fn float_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Float
}

fn string_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::String
}

fn bool_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Bool
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