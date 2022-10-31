use crate::types::*;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum BuiltInType {
    UnsignedInt8,
    SignedInt8,
    UnsignedInt16,
    SignedInt16,
    UnsignedInt32,
    SignedInt32,
    UnsignedInt64,
    SignedInt64,
    Float32,
    Float64,
    String,
    Boolean,
    Void
}

pub fn unsigned_int_8_built_in_type() -> BuiltInType {
    BuiltInType::UnsignedInt8
}

pub fn signed_int_8_built_in_type() -> BuiltInType {
    BuiltInType::SignedInt8
}

pub fn unsigned_int_16_built_in_type() -> BuiltInType {
    BuiltInType::UnsignedInt16
}

pub fn signed_int_16_built_in_type() -> BuiltInType {
    BuiltInType::SignedInt16
}

pub fn unsigned_int_32_built_in_type() -> BuiltInType {
    BuiltInType::UnsignedInt32
}

pub fn signed_int_32_built_in_type() -> BuiltInType {
    BuiltInType::SignedInt32
}

pub fn unsigned_int_64_built_in_type() -> BuiltInType {
    BuiltInType::UnsignedInt64
}

pub fn signed_int_64_built_in_type() -> BuiltInType {
    BuiltInType::SignedInt64
}

pub fn float_32_built_in_type() -> BuiltInType {
    BuiltInType::Float32
}

pub fn float_64_built_in_type() -> BuiltInType {
    BuiltInType::Float64
}

pub fn string_built_in_type() -> BuiltInType {
    BuiltInType::String
}

pub fn bool_built_in_type() -> BuiltInType {
    BuiltInType::Boolean
}

pub fn void_built_in_type() -> BuiltInType {
    BuiltInType::Void
}

pub fn try_get_built_in_type(id: &RuntimeTypeId) -> Option<(BuiltInType, bool)> {
    if let RuntimeTypeId::BuiltInType { built_in_type, is_pointer } = id {
        return Some((*built_in_type, *is_pointer));
    }
    None
}