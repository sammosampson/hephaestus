use crate::parsing::*;
use crate::typing::*;

#[derive(PartialEq, Debug, Clone)]
pub struct SourceToken {
    pub position: SourceFilePosition,
    pub item: SourceTokenItem
}

pub fn create_token(position: SourceFilePosition, item: SourceTokenItem) -> SourceToken {
    SourceToken { position, item }
}

#[derive(PartialEq, Debug, Clone)]
pub enum SourceTokenItem {
    Directive(Directive),
    Pointer,
    Identifier(String),
    Type(BuiltInType),
    Keyword(Keyword),
    Enclosure(Enclosure),
    Range(Range),
    Operator(Operator),
    Assignment(Assignment),
    Literal(UnresolvedLiteral),
    Terminator(Terminator),
    Error(SourceTokenError),
    Eof
}

pub fn create_eof_token_item() -> SourceTokenItem {
    SourceTokenItem::Eof
}