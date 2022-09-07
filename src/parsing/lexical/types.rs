use crate::parsing::*;
use crate::typing::*;

type TypeOption = Option<ResolvableType>;

pub fn try_get_type(item: &SourceTokenItem) -> TypeOption {
    match item {
        SourceTokenItem::Type(t) => Some(resolved_resolvable_type(built_in_type_resolved_type_id(*t))),
        SourceTokenItem::Identifier(name) => Some(unresolved_named_resolvable_type(name.clone())),
        _ => None
    }
}

type BuiltInTypeOption = Option<BuiltInType>;

const SOURCE_TYPE_INT: &str = "int";
const SOURCE_TYPE_FLOAT: &str = "float";
const SOURCE_TYPE_VOID: &str = "void";

pub fn parse_built_in_type(from: &str) -> BuiltInTypeOption {
    if from == SOURCE_TYPE_INT {
        return Some(int_32_built_in_type());
    }
    if from == SOURCE_TYPE_FLOAT {
        return Some(float_32_built_in_type());
    }
    if from == SOURCE_TYPE_VOID {
        return Some(void_built_in_type());
    }
    None
}

pub fn create_type_token_item(built_in_type: BuiltInType) -> SourceTokenItem {
    SourceTokenItem::Type(built_in_type)
}