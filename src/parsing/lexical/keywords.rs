use crate::parsing::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Keyword {
    For,
    Return
}

pub fn create_keyword_token_item(keyword: Keyword) -> SourceTokenItem {
    SourceTokenItem::Keyword(keyword)
}


type ParseKeywordOption = Option<Keyword>;

const SOURCE_KEYWORD_FOR: &str = "for";
const SOURCE_KEYWORD_RETURN: &str = "return";

pub fn parse_keyword(from: &str) -> ParseKeywordOption {
    if from == SOURCE_KEYWORD_FOR {
        return Some(Keyword::For);
    }
    if from == SOURCE_KEYWORD_RETURN {
        return Some(Keyword::Return);
    }
    None
}