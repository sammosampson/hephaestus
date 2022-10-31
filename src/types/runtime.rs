use std::mem::*;

use crate::{
    parsing::*,
    types::*,
    threading::*,
    utilities::*
};

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


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct RuntimeTypeMember {
    pub name: String,
    pub field_type: RuntimeTypePointer,
}

fn runtime_type_member(name: String, field_type: RuntimeTypePointer) -> RuntimeTypeMember {
    RuntimeTypeMember {
        name,
        field_type
    }
}

pub type RuntimeTypeMembers = Vec<RuntimeTypeMember>;

fn string_runtime_type_members() -> RuntimeTypeMembers {
    vec!(
        runtime_type_member(string("count"), create_shareable(signed_int_64_runtime_type())),
        runtime_type_member(string("data"), create_shareable(unsigned_int_8_pointer_runtime_type()))
    )
}

pub fn get_type_of_member_by_member_name(fields: &RuntimeTypeMembers, name: &str) -> OptionalRuntimeTypePointer {
    let member = fields
        .iter()
        .filter(|member| member.name == name)
        .next();

    if let Some(field) = member {
        return Some(field.field_type.clone());
    }
    None
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct RuntimeType {
    pub id: RuntimeTypeId,
    pub name: String,
    pub item: RuntimeTypeItem,
    pub size: TypeSize
}

pub type RuntimeTypePointer = Shareable<RuntimeType>;
pub type OptionalRuntimeTypePointer = Option<RuntimeTypePointer>;
pub type RuntimeTypePointers = Vec<RuntimeTypePointer>;

pub fn create_type(id: RuntimeTypeId, name: String, item: RuntimeTypeItem, size: TypeSize) -> RuntimeType {
    RuntimeType {
        id: id,
        name,
        item,
        size
    }
}

pub fn unsigned_int_8_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(unsigned_int_8_built_in_type()),
        SOURCE_TYPE_U8.to_string(),
        unsigned_integer_type_item(),
        resolved_type_size(1)
    )
}

pub fn signed_int_8_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(signed_int_8_built_in_type()),
        SOURCE_TYPE_S8.to_string(),
        signed_integer_type_item(),
        resolved_type_size(1)
    )
}

pub fn unsigned_int_16_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(unsigned_int_16_built_in_type()),
        SOURCE_TYPE_U16.to_string(),
        unsigned_integer_type_item(),
        resolved_type_size(2)
    )
}

pub fn signed_int_16_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(signed_int_16_built_in_type()),
        SOURCE_TYPE_S16.to_string(),
        signed_integer_type_item(),
        resolved_type_size(2)
    )
}

pub fn unsigned_int_32_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(unsigned_int_32_built_in_type()),
        SOURCE_TYPE_U32.to_string(),
        unsigned_integer_type_item(),
        resolved_type_size(4)
    )
}

pub fn signed_int_32_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(signed_int_32_built_in_type()),
        SOURCE_TYPE_S32.to_string(),
        signed_integer_type_item(),
        resolved_type_size(4)
    )
}

pub fn unsigned_int_64_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(unsigned_int_64_built_in_type()),
        SOURCE_TYPE_U64.to_string(),
        unsigned_integer_type_item(),
        resolved_type_size(8)
    )
}

pub fn signed_int_64_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(signed_int_64_built_in_type()),
        SOURCE_TYPE_S64.to_string(),
        signed_integer_type_item(),
        resolved_type_size(8)
    )
}

pub fn float_32_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(float_32_built_in_type()),
        SOURCE_TYPE_F32.to_string(),
        float_type_item(),
        resolved_type_size(4)
    )
}

pub fn float_64_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(float_64_built_in_type()),
        SOURCE_TYPE_F64.to_string(),
        float_type_item(),
        resolved_type_size(8)
    )
}

pub fn string_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(string_built_in_type()),
        SOURCE_TYPE_STRING.to_string(),
        string_type_item(string_runtime_type_members()),
        resolved_type_size(16)
    )
}

pub fn bool_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(bool_built_in_type()),
        SOURCE_TYPE_BOOL.to_string(),
        bool_type_item(),
        resolved_type_size(1)
    )
}

pub fn void_runtime_type() -> RuntimeType {
    create_type(
        built_in_type_runtime_type_id(void_built_in_type()),
        SOURCE_TYPE_VOID.to_string(),
        void_type_item(),
        not_required_type_size()
    )
}

pub fn unsigned_int_8_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(unsigned_int_8_built_in_type()),
        unsigned_int_8_runtime_type()
    )
}

pub fn signed_int_8_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(signed_int_8_built_in_type()),
        signed_int_8_runtime_type()
    )
}

pub fn unsigned_int_16_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(unsigned_int_16_built_in_type()),
        unsigned_int_16_runtime_type()
    )
}

pub fn signed_int_16_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(signed_int_16_built_in_type()),
        signed_int_16_runtime_type()
    )
}

pub fn unsigned_int_32_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(unsigned_int_32_built_in_type()),
        unsigned_int_32_runtime_type()
    )
}

pub fn signed_int_32_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(signed_int_32_built_in_type()),
        signed_int_32_runtime_type()
    )
}

pub fn unsigned_int_64_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(unsigned_int_64_built_in_type()),
        unsigned_int_64_runtime_type()
    )
}

pub fn signed_int_64_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(signed_int_64_built_in_type()),
        signed_int_64_runtime_type()
    )
}

pub fn float_32_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(float_32_built_in_type()),
        float_32_runtime_type()
    )
}

pub fn float_64_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(float_64_built_in_type()),
        float_64_runtime_type()
    )
}

pub fn string_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(string_built_in_type()),
        string_runtime_type()
    )
}

pub fn bool_pointer_runtime_type() -> RuntimeType {
    pointer_runtime_type(
        built_in_type_pointer_runtime_type_id(bool_built_in_type()),
        bool_runtime_type()
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
    ConstantDefinition { constant_type: RuntimeTypePointer },
    Pointer { to_type: Box<RuntimeType> },
    Int { is_signed: bool },
    Float,
    String { members: RuntimeTypeMembers },
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

pub fn constant_definition_type_item(constant_type: RuntimeTypePointer) -> RuntimeTypeItem {
    RuntimeTypeItem::ConstantDefinition { constant_type }
}

pub fn pointer_type_item(to_type: Box<RuntimeType>) -> RuntimeTypeItem {
    RuntimeTypeItem::Pointer { to_type }
}

fn signed_integer_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Int { is_signed: true }
}

fn unsigned_integer_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Int { is_signed: false }
}

fn float_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Float
}

fn string_type_item(fields: RuntimeTypeMembers) -> RuntimeTypeItem {
    RuntimeTypeItem::String { members: fields }
}

fn bool_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Bool
}

fn void_type_item() -> RuntimeTypeItem {
    RuntimeTypeItem::Void
}

pub fn try_get_procedure_definition_runtime_type_item(item: &RuntimeTypeItem) -> Option<(RuntimeTypePointers, RuntimeTypePointers )> {
    if let RuntimeTypeItem::ProcedureDefinition { return_types, arg_types } = item {
       return Some((arg_types.clone(), return_types.clone()));
    }
    None
}

pub fn try_get_constant_definition_runtime_type_item(item: &RuntimeTypeItem) -> Option<RuntimeTypePointer> {
    if let RuntimeTypeItem::ConstantDefinition { constant_type } = item {
       return Some(constant_type.clone());
    }
    None
}