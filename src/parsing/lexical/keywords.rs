use crate::parsing::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Keyword {
    For,
}

pub fn create_keyword_token_item(keyword: Keyword) -> SourceTokenItem {
    SourceTokenItem::Keyword(keyword)
}


type ParseKeywordOption = Option<Keyword>;

const SOURCE_KEYWORD_FOR: &str = "for";

pub fn parse_keyword(from: &str) -> ParseKeywordOption {
    if from == SOURCE_KEYWORD_FOR {
        return Some(Keyword::For);
    }
    None
}