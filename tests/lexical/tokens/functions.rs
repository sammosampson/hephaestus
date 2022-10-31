use rust_hephaestus::*;

#[test]
fn compound_get_for_function_declaration() {
    let mut lexer = lex("SomeFunction :: (x: float) -> int {
    return 1;
}");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("SomeFunction".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Declaration));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float32));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::GoesTo));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt64));
       
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Keyword(Keyword::Return));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("1"))));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close)));
}

#[test]
fn compound_get_for_function_declaration_with_body() {
    let mut lexer = lex("SomeFunction :: () {
    x := 1;
}");
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("SomeFunction".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Declaration));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close)));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::InitialiseAssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("1"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close)));
}


#[test]
fn compound_get_for_function_declaration_two_args() {
    let mut lexer = lex("SomeFunction :: (a: int, b: float) -> void { }");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("SomeFunction".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Declaration));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("a".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt64));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Arg));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("b".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float32));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(Assignment::GoesTo));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Void));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close)));

}



#[test]
fn compound_get_for_function_declaration_pointer_args() {
    let mut lexer = lex("SomeFunction :: (a: *int) { }");

    eat_next_token(&mut lexer);
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt64));
}

#[test]
fn compound_get_for_function_declaration_other_built_in_type_args() {
    let mut lexer = lex("SomeFunction :: (a: u8, b: s8, c: u16, d: s16, e: u32, f: s32, g: u64, h: s64, g: float32, h: float64, i: bool) { }");

    eat_next_token(&mut lexer);
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt8));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt8));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt16));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt16));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt32));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt32));
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt64));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt64));
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float32));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float64));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Boolean));

}


#[test]
fn compound_get_for_function_declaration_other_built_in_pointer_type_args() {
    let mut lexer = lex("SomeFunction :: (a: *u8, b: *s8, c: *u16, d: *s16, e: *u32, f: *s32, g: *u64, h: *s64, g: *float32, h: *float64, i: *bool) { }");

    eat_next_token(&mut lexer);
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt8));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt8));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt16));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt16));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt32));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt32));
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);   
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer); 
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::UnsignedInt64));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::SignedInt64));
    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float32));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float64));

    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
    eat_next_token(&mut lexer);    
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Pointer);

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Boolean));

}


#[test]
fn compound_get_for_function_call() {
    let mut lexer = lex("main(a, 2);");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("main".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("a".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Arg));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(UnresolvedLiteral::Int(string("2"))));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));

}