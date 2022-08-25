#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Terminator {
    Line,
    Arg
}

pub fn create_line_terminator() -> Terminator {
    Terminator::Line
}

pub fn create_arg_terminator() -> Terminator {
    Terminator::Arg
}