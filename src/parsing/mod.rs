mod lexical;
mod syntactical;
mod source_files;

pub use syntactical::*;
pub use lexical::*;
pub use source_files::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Subtract
}

pub fn create_add_operator() -> Operator {
    Operator::Add
}

pub fn create_subtract_operator() -> Operator {
    Operator::Subtract
}


#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(usize),
    String(String)
}