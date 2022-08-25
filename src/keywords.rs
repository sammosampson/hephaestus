#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Keyword {
    For,
    Int,
    Float
}

type ParseKeywordOption = Option<Keyword>;

const SOURCE_KEYWORD_FOR: &str = "for";
const SOURCE_KEYWORD_INT: &str = "int";
const SOURCE_KEYWORD_FLOAT: &str = "float";

pub fn parse_keyword(from: &str) -> ParseKeywordOption {
    if from == SOURCE_KEYWORD_FOR {
        return Some(Keyword::For);
    }
    if from == SOURCE_KEYWORD_INT {
        return Some(Keyword::Int);
    }
    if from == SOURCE_KEYWORD_FLOAT {
        return Some(Keyword::Float);
    }
    None
}