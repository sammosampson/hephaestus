use crate::utilities::*;
use std::str::*;

pub type ByteString = Vec<u8>;

pub fn to_byte_string(from: &str) -> ByteString {
    let mut byte_string = vec!();
    let mut is_slashed = false;
    for character in from.chars() {
        if character == '\\' {
            is_slashed = true;
            continue;
        }

        if !is_slashed {
            byte_string.push(character as u8);
            continue;
        }

        is_slashed = false;

        if character == 'r' {
            byte_string.push(13);
            continue;
        }

        if character == 'n' {
            byte_string.push(10);
            continue;
        }

        if character.is_numeric() {
            byte_string.push(character.to_digit(10).unwrap() as u8);
            continue;
        }
    }

    byte_string
}

pub fn byte_string_to_string(from: &ByteString) -> String {
    string(from_utf8(from).unwrap())
}