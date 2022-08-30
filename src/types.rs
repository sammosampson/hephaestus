use crate::tokenisation::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    BuiltIn(BuiltInType),
    Compound(String)
}


type TypeOption = Option<Type>;

pub fn try_get_type(item: &SourceTokenItem) -> TypeOption {
    match item {
        SourceTokenItem::Type(t) => Some(Type::BuiltIn(*t)),
        SourceTokenItem::Identifier(name) => Some(Type::Compound(name.clone())),
        _ => None
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BuiltInType {
    Int,
    Float,
    Void
}

type BuiltInTypeOption = Option<BuiltInType>;

const SOURCE_TYPE_INT: &str = "int";
const SOURCE_TYPE_FLOAT: &str = "float";
const SOURCE_TYPE_VOID: &str = "void";

pub fn parse_built_in_type(from: &str) -> BuiltInTypeOption {
    if from == SOURCE_TYPE_INT {
        return Some(BuiltInType::Int);
    }
    if from == SOURCE_TYPE_FLOAT {
        return Some(BuiltInType::Float);
    }
    if from == SOURCE_TYPE_VOID {
        return Some(BuiltInType::Void);
    }
    None
}