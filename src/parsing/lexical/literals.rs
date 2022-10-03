use crate::parsing::*;

pub fn create_int_literal_token_item(number: usize, is_negative: bool) -> SourceTokenItem {
    SourceTokenItem::Literal(unresolved_int_literal(number, is_negative))
}

pub fn create_float_literal_token_item(number: f64, is_negative: bool) -> SourceTokenItem {
    SourceTokenItem::Literal(unresolved_float_literal(number, is_negative))
}

pub fn create_string_literal_token_item(string: String) -> SourceTokenItem {
    SourceTokenItem::Literal(unresolved_string_literal(string))
}

pub fn try_get_string_literal(item: &SourceTokenItem) -> Option<String> {
    if let SourceTokenItem::Literal(UnresolvedLiteral::String(string)) = item {
       return Some(string.clone());
    }
    None
}