use crate::parsing::*;

pub fn create_identifier_token_item(name: String) -> SourceTokenItem {
    SourceTokenItem::Identifier(name)
}

pub fn create_period_token_item() -> SourceTokenItem {
    SourceTokenItem::Period
}

pub fn is_period(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Period
}
