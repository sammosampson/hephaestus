use std::num::*;
use crate::parsing::*;

type ParseNumberResult = Result<usize, ParseIntError>;

pub fn parse_number(from: &str) -> ParseNumberResult {
    from.parse::<usize>()
}

pub fn create_number_literal_token_item(number: usize) -> SourceTokenItem {
    SourceTokenItem::Literal(Literal::Int(number))
}

pub fn create_string_literal_token_item(string: String) -> SourceTokenItem {
    SourceTokenItem::Literal(Literal::String(string))
}

pub fn try_get_string_literal(item: &SourceTokenItem) -> Option<String> {
    if let SourceTokenItem::Literal(Literal::String(string)) = item {
       return Some(string.clone());
    }
    None
}