use crate::parsing::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Directive {
    Run,
    Load,
    ForeignSystemLibrary,
    Foreign
}

pub fn create_directive_token_item(directive: Directive) -> SourceTokenItem {
    SourceTokenItem::Directive(directive)
}

type ParseDirectiveOption = Option<Directive>;

const SOURCE_DIRECTIVE_RUN: &str = "run";
const SOURCE_DIRECTIVE_LOAD: &str = "load";
const SOURCE_DIRECTIVE_FOREIGN: &str = "foreign";
const SOURCE_DIRECTIVE_FOREIGN_SYSTEM_LIBRARY: &str = "foreign_system_library";

pub fn parse_directive_token_item(from: &str) -> ParseDirectiveOption {
    match from {
        SOURCE_DIRECTIVE_RUN => return Some(Directive::Run),
        SOURCE_DIRECTIVE_LOAD => return Some(Directive::Load),
        SOURCE_DIRECTIVE_FOREIGN => return Some(Directive::Foreign),
        SOURCE_DIRECTIVE_FOREIGN_SYSTEM_LIBRARY => return Some(Directive::ForeignSystemLibrary),
        _ => None
    }
}

pub fn is_foreign_directive(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Directive(Directive::Foreign)
}