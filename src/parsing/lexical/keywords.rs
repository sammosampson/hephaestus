use crate::parsing::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Keyword {
    Null,
    Struct,
    For,
    Cast,
    Return
}

pub fn create_keyword_token_item(keyword: Keyword) -> SourceTokenItem {
    SourceTokenItem::Keyword(keyword)
}


type ParseKeywordOption = Option<Keyword>;

const SOURCE_KEYWORD_NULL: &str = "null";
const SOURCE_KEYWORD_STRUCT: &str = "struct";
const SOURCE_KEYWORD_FOR: &str = "for";
const SOURCE_KEYWORD_CAST: &str = "cast";
const SOURCE_KEYWORD_RETURN: &str = "return";

pub fn parse_keyword(from: &str) -> ParseKeywordOption {
    if from == SOURCE_KEYWORD_STRUCT {
        return Some(Keyword::Struct);
    }
    if from == SOURCE_KEYWORD_NULL {
        return Some(Keyword::Null);
    }
    if from == SOURCE_KEYWORD_FOR {
        return Some(Keyword::For);
    }
    if from == SOURCE_KEYWORD_CAST {
        return Some(Keyword::Cast);
    }
    if from == SOURCE_KEYWORD_RETURN {
        return Some(Keyword::Return);
    }
    None
}

pub fn is_struct_keyword(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Keyword(Keyword::Struct)
}
