use crate::tokenisation::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Terminator {
    Line,
    Arg
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
