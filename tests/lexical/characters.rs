use rust_hephaestus::*;

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