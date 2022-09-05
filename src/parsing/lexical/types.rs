use crate::parsing::*;
use crate::typing::*;

type TypeOption = Option<ResolvableType>;

pub fn try_get_type(item: &SourceTokenItem) -> TypeOption {
    match item {
        SourceTokenItem::Type(t) => Some(ResolvableType::Resolved(ResolvedTypeId::BuiltInType(*t))),
        SourceTokenItem::Identifier(name) => Some(ResolvableType::UnresolvedNamed(name.clone())),
        _ => None
    }
}

type BuiltInTypeOption = Option<BuiltInType>;

const SOURCE_TYPE_INT: &str = "int";
const SOURCE_TYPE_FLOAT: &str = "float";
const SOURCE_TYPE_VOID: &str = "void";

pub fn parse_built_in_type(from: &str) -> BuiltInTypeOption {
    if from == SOURCE_TYPE_INT {
        return Some(BuiltInType::Int32);
    }
    if from == SOURCE_TYPE_FLOAT {
        return Some(BuiltInType::Float32);
    }
    if from == SOURCE_TYPE_VOID {
        return Some(BuiltInType::Void);
    }
    None
}

pub fn create_type_token_item(built_in_type: BuiltInType) -> SourceTokenItem {
    SourceTokenItem::Type(built_in_type)
}