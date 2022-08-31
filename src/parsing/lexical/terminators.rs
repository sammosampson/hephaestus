use crate::parsing::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Terminator {
    Line,
    Arg
}

pub fn create_terminator_token_item(terminator: Terminator) -> SourceTokenItem {
    SourceTokenItem::Terminator(terminator)
}

pub fn create_line_terminator() -> Terminator {
    Terminator::Line
}

pub fn create_arg_separator() -> Terminator {
    Terminator::Arg
}

pub fn is_arg_separator(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Terminator(Terminator::Arg)
}
