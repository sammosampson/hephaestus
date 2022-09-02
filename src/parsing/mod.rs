mod lexical;
mod syntactical;
mod source_files;

pub use syntactical::*;
pub use lexical::*;
pub use source_files::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    BuiltIn(BuiltInType),
    Compound(String),
    Unknown
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BuiltInType {
    Int,
    Float,
    Void
}

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