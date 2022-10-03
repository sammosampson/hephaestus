use crate::parsing::*;
use crate::threading::create_shareable;
use crate::typing::*;

type TypeOption = Option<ResolvableType>;

pub fn try_get_type(item: &SourceTokenItem, is_pointer: bool) -> TypeOption {
    match item {
        SourceTokenItem::Type(t) => Some(resolved_resolvable_type(create_shareable(to_runtime_type(*t, is_pointer)))),
        SourceTokenItem::Identifier(name) => Some(unresolved_named_resolvable_type(name.clone())),
        _ => None
    }
}
fn to_runtime_type(from: BuiltInType, is_pointer: bool) -> RuntimeType {
    if !is_pointer {
        return to_runtime_non_pointer_type(from)
    }

    to_runtime_pointer_type(from)
}

fn to_runtime_non_pointer_type(from: BuiltInType) -> RuntimeType {
    match from {
        BuiltInType::UnsignedInt8 => unsigned_int_8_runtime_type(),
        BuiltInType::SignedInt8 => signed_int_8_runtime_type(),
        BuiltInType::UnsignedInt16 => unsigned_int_16_runtime_type(),
        BuiltInType::SignedInt16 => signed_int_16_runtime_type(),
        BuiltInType::UnsignedInt32 => unsigned_int_32_runtime_type(),
        BuiltInType::SignedInt32 => signed_int_32_runtime_type(),
        BuiltInType::UnsignedInt64 => unsigned_int_64_runtime_type(),
        BuiltInType::SignedInt64 => signed_int_64_runtime_type(),
        BuiltInType::Float32 => float_32_runtime_type(),
        BuiltInType::Float64 => float_64_runtime_type(),
        BuiltInType::String => string_runtime_type(),
        BuiltInType::Void => void_runtime_type(),
        BuiltInType::Boolean => bool_runtime_type(),
    }
}

fn to_runtime_pointer_type(from: BuiltInType) -> RuntimeType {
    match from {
        BuiltInType::UnsignedInt8 => unsigned_int_8_pointer_runtime_type(),
        BuiltInType::SignedInt8 => signed_int_8_pointer_runtime_type(),
        BuiltInType::UnsignedInt16 => unsigned_int_16_pointer_runtime_type(),
        BuiltInType::SignedInt16 => signed_int_16_pointer_runtime_type(),
        BuiltInType::UnsignedInt32 => unsigned_int_32_pointer_runtime_type(),
        BuiltInType::SignedInt32 => signed_int_32_pointer_runtime_type(),
        BuiltInType::UnsignedInt64 => unsigned_int_64_pointer_runtime_type(),
        BuiltInType::SignedInt64 => signed_int_64_pointer_runtime_type(),
        BuiltInType::Float32 => float_32_pointer_runtime_type(),
        BuiltInType::Float64 => float_64_pointer_runtime_type(),
        BuiltInType::String => string_pointer_runtime_type(),
        BuiltInType::Void => void_pointer_runtime_type(),
        BuiltInType::Boolean => bool_pointer_runtime_type(),
    }
}

type BuiltInTypeOption = Option<BuiltInType>;

pub const SOURCE_TYPE_INT: &str = "int";
pub const SOURCE_TYPE_FLOAT: &str = "float";
pub const SOURCE_TYPE_U8: &str = "u8";
pub const SOURCE_TYPE_S8: &str = "s8";
pub const SOURCE_TYPE_U16: &str = "u16";
pub const SOURCE_TYPE_S16: &str = "s16";
pub const SOURCE_TYPE_U32: &str = "u32";
pub const SOURCE_TYPE_S32: &str = "s32";
pub const SOURCE_TYPE_U64: &str = "u64";
pub const SOURCE_TYPE_S64: &str = "s64";
pub const SOURCE_TYPE_F32: &str = "float32";
pub const SOURCE_TYPE_F64: &str = "float64";
pub const SOURCE_TYPE_VOID: &str = "void";
pub const SOURCE_TYPE_STRING: &str = "string";
pub const SOURCE_TYPE_BOOL: &str = "bool";

pub fn parse_built_in_type(from: &str) -> BuiltInTypeOption {
    match from {
        SOURCE_TYPE_U8 => Some(unsigned_int_8_built_in_type()), 
        SOURCE_TYPE_S8 => Some(signed_int_8_built_in_type()), 
        SOURCE_TYPE_U16 => Some(unsigned_int_16_built_in_type()), 
        SOURCE_TYPE_S16 => Some(signed_int_16_built_in_type()), 
        SOURCE_TYPE_U32 => Some(unsigned_int_32_built_in_type()), 
        SOURCE_TYPE_S32 => Some(signed_int_32_built_in_type()), 
        SOURCE_TYPE_U64 => Some(unsigned_int_64_built_in_type()), 
        SOURCE_TYPE_S64 => Some(signed_int_64_built_in_type()), 
        SOURCE_TYPE_INT => Some(signed_int_64_built_in_type()),
        SOURCE_TYPE_FLOAT => Some(float_32_built_in_type()),
        SOURCE_TYPE_F32 => Some(float_32_built_in_type()),
        SOURCE_TYPE_F64 => Some(float_64_built_in_type()),
        SOURCE_TYPE_STRING => Some(string_built_in_type()),
        SOURCE_TYPE_BOOL => Some(bool_built_in_type()),
        SOURCE_TYPE_VOID => Some(void_built_in_type()),
        _=> None,
    }
}

pub fn create_type_token_item(built_in_type: BuiltInType) -> SourceTokenItem {
    SourceTokenItem::Type(built_in_type)
}

pub fn create_pointer_token_item() -> SourceTokenItem {
    SourceTokenItem::Pointer
}

pub fn is_pointer(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Pointer
}
