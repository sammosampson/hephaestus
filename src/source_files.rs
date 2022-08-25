use std::{ iter::*, str::* };

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

type CharacterEnumerator<'a> = Enumerate<Chars<'a>>;
const CHARACTER_LF: char = '\n';
    
#[derive(Clone)]
pub struct SourceFileCharacterReader<'a> {
    characters: CharacterEnumerator<'a>,
    cursor: Option<SourceFilePosition>
}

pub fn create_reader(input: &str) -> SourceFileCharacterReader {
    SourceFileCharacterReader {
        characters: input.chars().enumerate(),
        cursor: None
    }
}

pub fn peek_next_character(reader: &SourceFileCharacterReader) -> SourceFileCharacter {
    let mut reader = reader.clone();
    get_next_character(&mut reader)
}

fn get_next_character(reader: &mut SourceFileCharacterReader) -> SourceFileCharacter {
    if let Some((_, value)) = reader.characters.next() {        
        if let Some(previous_position) = reader.cursor {            
            if value == CHARACTER_LF {
                reader.cursor = Some(increment_source_file_position_line(previous_position));
                return get_next_character(reader);
            } else {            
                reader.cursor = Some(increment_source_file_position_col(previous_position));
            }
        } else {
            reader.cursor = Some(first_character_in_source_file_position());
        }
    
        return create_valid_character(reader.cursor.unwrap(), value)
    }

    create_eof_character()
}

pub fn eat_next_character(reader: &mut SourceFileCharacterReader) {
    get_next_character(reader);
}

pub fn eat_white_space(reader: &mut SourceFileCharacterReader) {
    if is_character_whitespace(&peek_next_character(reader)) {
        eat_next_character(reader);
    }
}

pub fn read_characters_up_until<T: Fn(&SourceFileCharacter) -> bool>(reader: &mut SourceFileCharacterReader, until: T) -> String {
    let mut result = String::default();

    loop {
        let character = peek_next_character(reader);

        if is_character_eof(&character) || until(&character) {
            break;
        }

        eat_next_character(reader);
        
        result.push(get_unwrapped_character_value(&character));
    }

    result
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SourceFileCharacter {
    Valid { position: SourceFilePosition, value: char },
    Eof
}

fn create_valid_character(position: SourceFilePosition, value: char) -> SourceFileCharacter {
    SourceFileCharacter::Valid {
        position,
        value
    }
}

fn create_eof_character() -> SourceFileCharacter {
    SourceFileCharacter::Eof
}

fn get_character_value(character: &SourceFileCharacter) -> Option<char> {
    match character {
        SourceFileCharacter::Valid { value, .. } => Some(*value),
        SourceFileCharacter::Eof => None
    }
}

pub fn get_unwrapped_character_value(character: &SourceFileCharacter) -> char {
    get_character_value(character).unwrap()
}

pub fn get_character_position(character: &SourceFileCharacter) -> SourceFilePosition {
    match character {
        SourceFileCharacter::Valid { position, .. } => *position,
        SourceFileCharacter::Eof => SourceFilePosition::default()
    }
}

pub fn is_character_eof(character: &SourceFileCharacter) -> bool {
    character == &SourceFileCharacter::Eof
}

pub fn is_character(character: &SourceFileCharacter, to_check: char) -> bool {
    if let Some(character) = get_character_value(character) {
        return character == to_check;
    }
    false
}

pub fn is_character_alphanumeric(character: &SourceFileCharacter) -> bool {
    if let Some(character) = get_character_value(character) {
        return character.is_alphanumeric();
    }
    false
}

pub fn is_character_whitespace(character: &SourceFileCharacter) -> bool {
    if let Some(character) = get_character_value(character) {
        return character.is_whitespace();
    }
    false
}


#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct SourceFilePosition {
    pub absolute: usize,
    pub line: usize,
    pub col: usize,
}

fn create_source_file_position(absolute: usize, line: usize, col: usize) -> SourceFilePosition {
    SourceFilePosition { absolute, line, col }
}

pub fn empty_position() -> SourceFilePosition {
    SourceFilePosition::default()
}

fn first_character_in_source_file_position() -> SourceFilePosition {
    create_source_file_position(0, 1, 1)
}

fn increment_source_file_position_col(position: SourceFilePosition) -> SourceFilePosition {
    create_source_file_position(position.absolute + 1, position.line, position.col + 1)
}

fn increment_source_file_position_line(position: SourceFilePosition) -> SourceFilePosition {
    create_source_file_position(position.absolute + 1, position.line + 1, 0)
}
