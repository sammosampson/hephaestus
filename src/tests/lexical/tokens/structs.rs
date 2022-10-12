use crate::parsing::*;
use crate::typing::*;

#[test]
fn compound_get_for_struct_declaration() {
    let mut lexer = lex("SomeStruct :: struct {
    x: float;
    y: int;
}");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("SomeStruct".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Declaration));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Keyword(Keyword::Struct));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float32));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("y".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt64));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
       
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close)));
}

#[test]
fn compound_get_for_built_in_type_struct_declaration() {
    let mut lexer = lex("string :: struct {
    len: int;
    data: *u8;
}");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::String));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Declaration));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Keyword(Keyword::Struct));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("len".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt64));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("data".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt8));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
       
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close)));
}