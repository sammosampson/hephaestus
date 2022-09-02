use crate::parsing::*;

#[test]
fn compound_get_for_directive_with_string_literal() {
    let mut lexer = lex("#load \"test.jai\";");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Directive(Directive::Load));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::String("test.jai".to_string())));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}