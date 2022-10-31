use crate::parsing::*;
use crate::utilities::*;
use crate::types::*;

#[test]
fn compound_get_for_typed_variable() {
    let mut lexer = lex("x: u32;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(unsigned_int_32_built_in_type()));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}

#[test]
fn compound_get_for_typed_variable_with_initial_asignment() {
    let mut lexer = lex("x: u32 = 1;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(unsigned_int_32_built_in_type()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::AssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("1"))));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}

#[test]
fn compound_get_for_variable_initial_assignment_to_positive_int() {
    let mut lexer = lex("x := 1;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::InitialiseAssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("1"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}

#[test]
fn compound_get_for_variable_initial_assignment_to_negative_int() {
    let mut lexer = lex("x := -1;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::InitialiseAssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("-1"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}


#[test]
fn compound_get_for_variable_initial_assignment_to_float() {
    let mut lexer = lex("x := 1.0;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::InitialiseAssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Float(string("1.0"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}

#[test]
fn compound_get_for_variable_reassignment() {
    let mut lexer = lex("x = x + 1;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::AssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Operator(Operator::Add));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("1"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}

#[test]
fn compound_get_for_typed_constant_with_initial_asignment() {
    let mut lexer = lex("STD_OUTPUT_HANDLE : s32 : -11;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("STD_OUTPUT_HANDLE".to_string()));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(signed_int_32_built_in_type()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("-11"))));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}

#[test]
fn compound_get_for_assignment_to_cast() {
    let mut lexer = lex("x := cast(s32) y;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::InitialiseAssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Keyword(Keyword::Cast));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Open)));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(signed_int_32_built_in_type()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("y".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}


#[test]
fn compound_get_for_assignment_to_string() {
    let mut lexer = lex("x := \"hello\\r\\n\\0\";");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::InitialiseAssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(unresolved_string_literal(vec!(104, 101, 108, 108, 111, 13, 10, 0))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}
 