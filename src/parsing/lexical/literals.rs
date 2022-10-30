use crate::{parsing::*, strings::*};

pub fn create_int_literal_token_item(number: String) -> SourceTokenItem {
    SourceTokenItem::Literal(unresolved_int_literal(number))
}

pub fn create_float_literal_token_item(number: String) -> SourceTokenItem {
    SourceTokenItem::Literal(unresolved_float_literal(number))
}

pub fn create_string_literal_token_item(string: ByteString) -> SourceTokenItem {
    SourceTokenItem::Literal(unresolved_string_literal(string))
}

pub fn try_get_string_literal(item: &SourceTokenItem) -> Option<ByteString> {
    if let SourceTokenItem::Literal(UnresolvedLiteral::String(string)) = item {
       return Some(string.clone());
    }
    None
}