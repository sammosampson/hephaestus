use std::{ iter::*, str::* };
use crate::parsing::*;

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

pub fn get_next_character(reader: &mut SourceFileCharacterReader) -> SourceFileCharacter {
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

pub fn get_character_value(character: &SourceFileCharacter) -> Option<char> {
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
