use crate::parsing::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Range {
    LeftInclusive,
}

pub fn create_range_token_item(range: Range) -> SourceTokenItem {
    SourceTokenItem::Range(range)
}

pub fn create_left_inclusive_range() -> Range {
    Range::LeftInclusive
}