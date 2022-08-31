use crate::parsing::*;

pub fn create_operator_token_item(op: Operator) -> SourceTokenItem {
    SourceTokenItem::Operator(op)
}
