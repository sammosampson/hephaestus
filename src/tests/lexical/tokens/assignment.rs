use crate::parsing::*;

#[test]
fn compound_get_for_variable_initial_assignment_to_positive_int() {
    let mut lexer = lex("x := 1;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::InitialiseAssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int { number: 1, is_negative: false }));

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
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int { number: 1, is_negative: true }));

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
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Float { number: 1.0, is_negative: false }));

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
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int { number: 1, is_negative: false }));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}
