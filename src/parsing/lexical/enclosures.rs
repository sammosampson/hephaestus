use super::tokenisation::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum EnclosureType {
    Open,
    Close
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Enclosure {
    Brace(EnclosureType),
    Parentheses(EnclosureType),
}

pub fn create_open_brace_enclosure() -> Enclosure {
    Enclosure::Brace(EnclosureType::Open)
}

pub fn create_open_parentheses_enclosure() -> Enclosure {
    Enclosure::Parentheses(EnclosureType::Open)
}

pub fn create_closed_brace_enclosure() -> Enclosure {
    Enclosure::Brace(EnclosureType::Close)
}

pub fn create_closed_parentheses_enclosure() -> Enclosure {
    Enclosure::Parentheses(EnclosureType::Close)
}

pub fn is_open_paren(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Open))
}

pub fn is_close_paren(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close))
}

pub fn is_open_brace(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open))
}

pub fn is_close_brace(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close))
}