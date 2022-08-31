use crate::parsing::*;

#[test]
fn peek_next_character_empty_input() {
    let reader = create_reader("");
    assert_eq!(is_character_eof(&peek_next_character(&reader)), true);
    assert_eq!(is_character_eof(&peek_next_character(&reader)), true);
}

#[test]
fn peek_next_character_with_input() {
    let reader = create_reader("ab");
    assert_eq!(get_unwrapped_character_value(&peek_next_character(&reader)), 'a');
    assert_eq!(get_unwrapped_character_value(&peek_next_character(&reader)), 'a');
}

#[test]
fn get_next_character_with_input() {
    let mut reader = create_reader("ab");
    assert_eq!(get_unwrapped_character_value(&get_next_character(&mut reader)), 'a');
    assert_eq!(get_unwrapped_character_value(&get_next_character(&mut reader)), 'b');
    let eof_char = get_next_character(&mut reader);
    assert_eq!(get_character_value(&eof_char), None);        
}

#[test]
fn get_next_character_with_multiline_input_sets_positioning_correctly() {
    let mut reader = create_reader("a b 
c d");

    let character = get_next_character(&mut reader);
    let value = get_unwrapped_character_value(&character);
    let position = get_character_position(&character);
    assert_eq!(value, 'a');
    assert_eq!(position.absolute, 0);
    assert_eq!(position.line, 1);
    assert_eq!(position.col, 1);

    eat_next_character(&mut reader);

    let character = get_next_character(&mut reader);
    let value = get_unwrapped_character_value(&character);
    let position = get_character_position(&character);
    assert_eq!(value, 'b');
    assert_eq!(position.absolute, 2);
    assert_eq!(position.line, 1);
    assert_eq!(position.col, 3);

    eat_next_character(&mut reader);

    let character = get_next_character(&mut reader);
    let value = get_unwrapped_character_value(&character);
    let position = get_character_position(&character);
    assert_eq!(value, 'c');
    assert_eq!(position.absolute, 5);
    assert_eq!(position.line, 2);
    assert_eq!(position.col, 1);

    eat_next_character(&mut reader);

    let character = get_next_character(&mut reader);
    let value = get_unwrapped_character_value(&character);
    let position = get_character_position(&character);
    assert_eq!(value, 'd');
    assert_eq!(position.absolute, 7);
    assert_eq!(position.line, 2);
    assert_eq!(position.col, 3);

    let character = get_next_character(&mut reader);
    let position = get_character_position(&character);
    assert_eq!(is_character_eof(&character), true);
    assert_eq!(position.absolute, 0);
    assert_eq!(position.line, 0);
    assert_eq!(position.col, 0);
}

#[test]
fn read_characters_up_until_with_whitespace_check() {
    let mut reader = create_reader("ab cd ef");

    let read_characters = read_characters_up_until(
        &mut reader, 
        |c| is_character_whitespace(c)
    );
    
    assert_eq!(&read_characters, "ab");

    eat_next_character(&mut reader);
    
    let read_characters = read_characters_up_until(
        &mut reader, 
        |c| is_character_whitespace(c)
    );
    
    assert_eq!(&read_characters, "cd");
}

#[test]
fn read_characters_up_until_stops_at_eof_regardless() {
    let mut reader = create_reader("ab");

    let read_characters = read_characters_up_until(
        &mut reader, 
        |c| is_character_whitespace(c)
    );
    
    assert_eq!(&read_characters, "ab");
}


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
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::Int(1)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 7);
    assert_eq!(token.position.col, 8);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Operator(Operator::Add));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 9);
    assert_eq!(token.position.col, 10);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::Int(2)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 11);
    assert_eq!(token.position.col, 12);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Operator(Operator::Subtract));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 13);
    assert_eq!(token.position.col, 14);
    assert_eq!(token.position.line, 1);
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::Int(5)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.position.absolute, 0);
    assert_eq!(token.position.col, 0);
    assert_eq!(token.position.line, 0);
    assert_eq!(token.item, SourceTokenItem::Eof);
}


#[test]
fn compound_get_for_variable_initial_assignment() {
    let mut lexer = lex("x := 1;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::InitialiseAssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::Int(1)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}

#[test]
fn compound_get_for_variable_reassignment() {
    let mut lexer = lex("x = x + 1;");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::AssignValue));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Operator(Operator::Add));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::Int(1)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
}

#[test]
fn compound_get_for_for_loop() {
    let mut lexer = lex("for x: 0..count { }");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Keyword(Keyword::For));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::Initialise));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::Int(0)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Range(Range::LeftInclusive));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("count".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close)));
}


#[test]
fn compound_get_for_function_declaration() {
    let mut lexer = lex("SomeFunction :: (x: float) -> void");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("SomeFunction".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::Declaration));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("x".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::GoesTo));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Void));
}

#[test]
fn compound_get_for_function_declaration_two_args() {
    let mut lexer = lex("SomeFunction :: (a: int, b: float) -> void { }");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("SomeFunction".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::Declaration));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("a".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Int));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Arg));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Identifier("b".to_string()));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::Initialise));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Float));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Assignment(AssignmentOperator::GoesTo));
    
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Type(BuiltInType::Void));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Open)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Brace(EnclosureType::Close)));

}


#[test]
fn compound_get_for_string_literal() {
    let mut lexer = lex("#load \"test.jai\";");

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Directive(Directive::Load));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::String("test.jai".to_string())));
        
    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));
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
    assert_eq!(token.item, SourceTokenItem::Literal(Literal::Int(2)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Enclosure(Enclosure::Parentheses(EnclosureType::Close)));

    let token = get_next_token(&mut lexer);
    assert_eq!(token.item, SourceTokenItem::Terminator(Terminator::Line));

}