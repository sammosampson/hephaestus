#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Directive {
    Run,
    Load
}

type ParseDirectiveOption = Option<Directive>;

const SOURCE_DIRECTIVE_RUN: &str = "run";
const SOURCE_DIRECTIVE_LOAD: &str = "load";

pub fn parse_directive(from: &str) -> ParseDirectiveOption {
    if from == SOURCE_DIRECTIVE_RUN {
        return Some(Directive::Run);
    }
    if from == SOURCE_DIRECTIVE_LOAD {
        return Some(Directive::Load);
    }
    None
}