use crate::parsing::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Directive {
    Run,
    Load
}

pub fn create_directive_token_item(directive: Directive) -> SourceTokenItem {
    SourceTokenItem::Directive(directive)
}

type ParseDirectiveOption = Option<Directive>;

const SOURCE_DIRECTIVE_RUN: &str = "run";
const SOURCE_DIRECTIVE_LOAD: &str = "load";

pub fn parse_directive_token_item(from: &str) -> ParseDirectiveOption {
    if from == SOURCE_DIRECTIVE_RUN {
        return Some(Directive::Run);
    }
    if from == SOURCE_DIRECTIVE_LOAD {
        return Some(Directive::Load);
    }
    None
}