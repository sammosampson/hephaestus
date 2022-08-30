use crate::tokenisation::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(usize),
    String(String)
}

pub fn try_get_string_literal(item: &SourceTokenItem) -> Option<String> {
    if let SourceTokenItem::Literal(Literal::String(string)) = item {
       return Some(string.clone());
    }
    None
}