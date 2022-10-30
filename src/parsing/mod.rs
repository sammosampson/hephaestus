mod lexical;
mod syntactical;
mod source_files;

pub use syntactical::*;
pub use lexical::*;
pub use source_files::*;

use crate::strings::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply
}

pub fn add_operator() -> Operator {
    Operator::Add
}

pub fn subtract_operator() -> Operator {
    Operator::Subtract
}

pub fn multiply_operator() -> Operator {
    Operator::Multiply
}

#[derive(PartialEq, Debug, Clone)]
pub enum Scope {
    Unknown,
    Local,
    Global
}

pub fn unknown_scope() -> Scope {
    Scope::Unknown
}

pub fn local_scope() -> Scope {
    Scope::Local
}

pub fn global_scope() -> Scope {
    Scope::Global
}

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvableLiteral {
    Unresolved(UnresolvedLiteral),
    Resolved(ResolvedLiteral),
}

pub fn unresolved_resolvable_literal(literal :UnresolvedLiteral) -> ResolvableLiteral {
    ResolvableLiteral::Unresolved(literal)
}

pub fn resolved_resolvable_literal(literal :ResolvedLiteral) -> ResolvableLiteral {
    ResolvableLiteral::Resolved(literal)
}

pub fn get_resolved_literal(literal: &ResolvableLiteral) -> ResolvedLiteral {
    if let ResolvableLiteral::Resolved(literal) = literal {
        return literal.clone();
    }
    panic!("resolvable literal is not resolved")
}

#[derive(PartialEq, Debug, Clone)]
pub enum UnresolvedLiteral {
    Int(String),
    Float(String),
    String(ByteString)
}

pub fn unresolved_int_literal(number: String) -> UnresolvedLiteral {
    UnresolvedLiteral::Int(number)
}

pub fn unresolved_float_literal(number: String) -> UnresolvedLiteral {
    UnresolvedLiteral::Float(number)
}

pub fn unresolved_string_literal(string: ByteString) -> UnresolvedLiteral {
    UnresolvedLiteral::String(string)
}

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvedLiteral {
    UnsignedInt8(u8),
    SignedInt8(i8),
    UnsignedInt16(u16),
    SignedInt16(i16),
    UnsignedInt32(u32),
    SignedInt32(i32),
    UnsignedInt64(u64),
    SignedInt64(i64),
    Float32(f32),
    Float64(f64),
    String(ByteString)
}

pub fn resolved_unsigned_int_8_literal(value: u8) -> ResolvedLiteral {
    ResolvedLiteral::UnsignedInt8(value)
}

pub fn resolved_signed_int_8_literal(value: i8) -> ResolvedLiteral {
    ResolvedLiteral::SignedInt8(value)
}

pub fn resolved_unsigned_int_16_literal(value: u16) -> ResolvedLiteral {
    ResolvedLiteral::UnsignedInt16(value)
}

pub fn resolved_signed_int_16_literal(value: i16) -> ResolvedLiteral {
    ResolvedLiteral::SignedInt16(value)
}

pub fn resolved_unsigned_int_32_literal(value: u32) -> ResolvedLiteral {
    ResolvedLiteral::UnsignedInt32(value)
}

pub fn resolved_signed_int_32_literal(value: i32) -> ResolvedLiteral {
    ResolvedLiteral::SignedInt32(value)
}

pub fn resolved_unsigned_int_64_literal(value: u64) -> ResolvedLiteral {
    ResolvedLiteral::UnsignedInt64(value)
}

pub fn resolved_signed_int_64_literal(value: i64) -> ResolvedLiteral {
    ResolvedLiteral::SignedInt64(value)
}

pub fn resolved_float_32_literal(value: f32) -> ResolvedLiteral {
    ResolvedLiteral::Float32(value)
}

pub fn resolved_float_64_literal(value: f64) -> ResolvedLiteral {
    ResolvedLiteral::Float64(value)
}

pub fn resolved_string_literal(value: ByteString) -> ResolvedLiteral {
    ResolvedLiteral::String(value)
}