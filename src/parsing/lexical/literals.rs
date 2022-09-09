use std::num::*;
use crate::parsing::*;

type ParseUnsignedIntResult = Result<u64, ParseIntError>;
type ParseFloatResult = Result<f64, ParseFloatError>;

pub fn parse_unsigned_int(from: &str) -> ParseUnsignedIntResult {
    from.parse::<u64>()
}

pub fn parse_float(from: &str) -> ParseFloatResult {
    from.parse::<f64>()
}

pub fn create_unsigned_int_literal_token_item(number: u64) -> SourceTokenItem {
    SourceTokenItem::Literal(unsigned_int_literal(number))
}

pub fn create_float_literal_token_item(number: f64) -> SourceTokenItem {
    SourceTokenItem::Literal(float_literal(number))
}

pub fn create_string_literal_token_item(string: String) -> SourceTokenItem {
    SourceTokenItem::Literal(string_literal(string))
}

pub fn try_get_string_literal(item: &SourceTokenItem) -> Option<String> {
    if let SourceTokenItem::Literal(Literal::String(string)) = item {
       return Some(string.clone());
    }
    None
}