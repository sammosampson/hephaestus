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

pub fn add_operator() -> Operator {
    Operator::Add
}

pub fn subtract_operator() -> Operator {
    Operator::Subtract
}


#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    UnsignedInt(u64),
    Float(f64),
    String(String)
}

pub fn unsigned_int_literal(number: u64) -> Literal {
    Literal::UnsignedInt(number)
}

pub fn float_literal(number: f64) -> Literal {
    Literal::Float(number)
}

pub fn string_literal(string: String) -> Literal {
    Literal::String(string)
}