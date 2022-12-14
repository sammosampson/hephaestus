use rust_hephaestus::*;

#[test]
fn compound_get_for_for_loop() {
    let mut lexer = lex("for x: 0..count { }");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Keyword(Keyword::For));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("0"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Range(Range::LeftInclusive));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("count".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close)));
}
