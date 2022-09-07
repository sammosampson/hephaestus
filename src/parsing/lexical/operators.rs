use crate::parsing::*;

pub fn create_operator_token_item(op: Operator) -> SourceTokenItem {
    SourceTokenItem::Operator(op)
}

pub fn is_operator(item: &SourceTokenItem) -> bool {
    if let SourceTokenItem::Operator(_op) = item {
       return true;
    }
    false
}

