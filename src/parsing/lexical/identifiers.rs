use crate::parsing::*;

pub fn create_identifier_token_item(name: String) -> SourceTokenItem {
    SourceTokenItem::Identifier(name)
}
