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
        BuiltInType::SignedInt32 => signed_int_32_runtime_type(),
        BuiltInType::Float32 => float_32_runtime_type(),
        BuiltInType::Void => void_runtime_type(),
    }
}

fn to_runtime_pointer_type(from: BuiltInType) -> RuntimeType {
    match from {
        BuiltInType::SignedInt32 => signed_int_32_pointer_runtime_type(),
        BuiltInType::Float32 => float_32_pointer_runtime_type(),
        BuiltInType::Void => void_pointer_runtime_type(),
    }
}

type BuiltInTypeOption = Option<BuiltInType>;

const SOURCE_TYPE_INT: &str = "int";
const SOURCE_TYPE_S32: &str = "s32";
const SOURCE_TYPE_FLOAT: &str = "float";
const SOURCE_TYPE_F32: &str = "f32";
const SOURCE_TYPE_VOID: &str = "void";

pub fn parse_built_in_type(from: &str) -> BuiltInTypeOption {
    match from {
        SOURCE_TYPE_S32 => Some(signed_int_32_built_in_type()), 
        SOURCE_TYPE_INT => Some(signed_int_32_built_in_type()),
        SOURCE_TYPE_FLOAT => Some(float_32_built_in_type()),
        SOURCE_TYPE_F32 => Some(float_32_built_in_type()),
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
