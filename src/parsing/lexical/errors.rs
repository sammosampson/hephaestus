use crate::parsing::*;

#[derive(PartialEq, Debug, Clone)]
pub enum SourceTokenError {
    UnknownToken(char),
    UnknownDirective(String)
}

pub fn create_error_token_item(error: SourceTokenError) -> SourceTokenItem {
    SourceTokenItem::Error(error)
}

pub fn create_unknown_token_error(token: char) -> SourceTokenError {
    SourceTokenError::UnknownToken(token)
}

pub fn create_unknown_directive_error(name: String) -> SourceTokenError {
    SourceTokenError::UnknownDirective(name)
}