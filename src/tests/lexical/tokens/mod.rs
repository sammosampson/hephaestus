mod assignment;
mod loops;
mod functions;
mod directives;

use crate::parsing::*;
use crate::utilities::*;

#[test]
fn peek_next_token_peeks_correctly() {
    let lexer = lex("#run 1 + 2");

    let peeked_token = peek_next_token(&lexer);
    assert_eq!(peeked_token.item, SourceTokenItem::Directive(Directive::Run));

    let peeked_token = peek_next_token(&lexer);
    assert_eq!(peeked_token.item, SourceTokenItem::Directive(Directive::Run));
}

#[test]
fn compound_get_next_token_gets_correctly_with_positioning() {
    let mut lexer = lex("#run 1 + 2 - 5");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 0);
    assert_eq!(token.position.col, 1);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Directive(Directive::Run));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 5);
    assert_eq!(token.position.col, 6);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("1"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 7);
    assert_eq!(token.position.col, 8);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Operator(Operator::Add));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 9);
    assert_eq!(token.position.col, 10);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("2"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 11);
    assert_eq!(token.position.col, 12);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Operator(Operator::Subtract));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 13);
    assert_eq!(token.position.col, 14);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("5"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 0);
    assert_eq!(token.position.col, 0);
    assert_eq!(token.position.line, 0);
    assert_eq!(token.item, SourceTokenItem::Eof);
}
