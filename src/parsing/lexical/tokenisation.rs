use std::num::*;

use super:: {
    source_files::*,
    literals::*,
    operators::*,
    keywords::*,
    ranges::*,
    enclosures::*,
    terminators::*,
    directives::*,
    types::*,
};
use crate::parsing::source_files::*;

#[derive(PartialEq, Debug, Clone)]
pub struct SourceToken {
    pub position: SourceFilePosition,
    pub item: SourceTokenItem
}

fn create_token(position: SourceFilePosition, item: SourceTokenItem) -> SourceToken {
    SourceToken { position, item }
}

#[derive(PartialEq, Debug, Clone)]
pub enum SourceTokenItem {
    Directive(Directive),
    Identifier(String),
    Type(BuiltInType),
    Keyword(Keyword),
    Enclosure(Enclosure),
    Range(Range),
    Operator(Operator),
    Assignment(AssignmentOperator),
    Literal(Literal),
    Terminator(Terminator),
    Error(SourceTokenError),
    Eof
}

fn create_eof_token_item() -> SourceTokenItem {
    SourceTokenItem::Eof
}

fn create_directive_token_item(directive: Directive) -> SourceTokenItem {
    SourceTokenItem::Directive(directive)
}

fn create_identifier_token_item(name: String) -> SourceTokenItem {
    SourceTokenItem::Identifier(name)
}

fn create_keyword_token_item(keyword: Keyword) -> SourceTokenItem {
    SourceTokenItem::Keyword(keyword)
}

fn create_type_token_item(built_in_type: BuiltInType) -> SourceTokenItem {
    SourceTokenItem::Type(built_in_type)
}

fn create_error_token_item(error: SourceTokenError) -> SourceTokenItem {
    SourceTokenItem::Error(error)
}

fn create_number_literal_token_item(number: usize) -> SourceTokenItem {
    SourceTokenItem::Literal(Literal::Int(number))
}

fn create_string_literal_token_item(string: String) -> SourceTokenItem {
    SourceTokenItem::Literal(Literal::String(string))
}

fn create_operator_token_item(op: Operator) -> SourceTokenItem {
    SourceTokenItem::Operator(op)
}

fn create_assignment_token_item(op: AssignmentOperator) -> SourceTokenItem {
    SourceTokenItem::Assignment(op)
}

fn create_range_token_item(range: Range) -> SourceTokenItem {
    SourceTokenItem::Range(range)
}

fn create_enclosure_token_item(enclosure: Enclosure) -> SourceTokenItem {
    SourceTokenItem::Enclosure(enclosure)
}

fn create_terminator_token_item(terminator: Terminator) -> SourceTokenItem {
    SourceTokenItem::Terminator(terminator)
}

#[derive(PartialEq, Debug, Clone)]
pub enum SourceTokenError {
    UnknownToken(char),
    UnknownDirective(String)
}

fn create_unknown_token_error(token: char) -> SourceTokenError {
    SourceTokenError::UnknownToken(token)
}

fn create_unknown_directive_error(name: String) -> SourceTokenError {
    SourceTokenError::UnknownDirective(name)
}

const SOURCE_SYMBOL_DIRECTIVE: char = '#';
const SOURCE_SYMBOL_SEMICOLON: char = ';';
const SOURCE_SYMBOL_COMMA: char = ',';
const SOURCE_SYMBOL_COLON: char = ':';
const SOURCE_SYMBOL_EQUALS: char = '=';
const SOURCE_SYMBOL_ADD: char = '+';
const SOURCE_SYMBOL_SUBTRACT: char = '-';
const SOURCE_SYMBOL_GREATER_THAN: char = '>';
const SOURCE_SYMBOL_PERIOD: char = '.';
const SOURCE_SYMBOL_OPEN_BRACE: char = '{';
const SOURCE_SYMBOL_CLOSE_BRACE: char = '}';
const SOURCE_SYMBOL_OPEN_PAREN: char = '(';
const SOURCE_SYMBOL_CLOSE_PAREN: char = ')';
const SOURCE_SYMBOL_QUOTES: char = '"';

#[derive(Clone)]
pub struct Lexer<'a> {
    reader: SourceFileCharacterReader<'a>,
}

pub fn lex(input: &str) -> Lexer {
    Lexer {
        reader: create_reader(input),
    }
}

pub fn get_next_token(lexer: &mut Lexer) -> SourceToken {
    read_next_token(lexer)
}

pub fn peek_next_token(lexer: &Lexer) -> SourceToken {
    let mut peek_lexer = lexer.clone();
    read_next_token(&mut peek_lexer)
}

pub fn eat_next_token(lexer: &mut Lexer) {
    get_next_token(lexer);
}

pub fn try_get_identifier(item: SourceTokenItem) -> Option<String> {
    if let SourceTokenItem::Identifier(name) = item {
       return Some(name);
    }
    None
}

fn read_next_token(lexer: &mut Lexer) -> SourceToken {
    eat_white_space(&mut lexer.reader);
    
    let next_character = peek_next_character(&lexer.reader);
    
    if is_character(&next_character, SOURCE_SYMBOL_SEMICOLON) {
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_terminator_token_item(create_line_terminator())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_COMMA) {
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_terminator_token_item(create_arg_separator())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_DIRECTIVE) {
        eat_next_character(&mut lexer.reader);
        let directive_name = read_up_until_non_alphanumeric(lexer);
        if let Some(directive) = parse_directive_token_item(&directive_name) {
            return create_token(
                get_character_position(&next_character), 
                create_directive_token_item(directive)
            );    
        }
        return create_token(
            get_character_position(&next_character), 
            create_error_token_item(create_unknown_directive_error(directive_name))
        )
    }
    
    if is_character(&next_character, SOURCE_SYMBOL_ADD) {
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_operator_token_item(create_add_operator())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_SUBTRACT) {
        eat_next_character(&mut lexer.reader);
        if is_character(&peek_next_character(&mut lexer.reader), SOURCE_SYMBOL_GREATER_THAN) {
            eat_next_character(&mut lexer.reader);
            return create_token(
                get_character_position(&next_character), 
                create_assignment_token_item(create_goes_to_assignment_operator())
            );
        }
        return create_token(
            get_character_position(&next_character), 
            create_operator_token_item(create_subtract_operator())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_EQUALS) {
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_assignment_token_item(create_assign_value_assignment_operator())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_COLON) {
        eat_next_character(&mut lexer.reader);
        if is_character(&peek_next_character(&mut lexer.reader), SOURCE_SYMBOL_EQUALS) {
            eat_next_character(&mut lexer.reader);
            return create_token(
                get_character_position(&next_character), 
                create_assignment_token_item(create_initialise_assign_value_assignment_operator())
            );
        }

        if is_character(&peek_next_character(&mut lexer.reader), SOURCE_SYMBOL_COLON) {
            eat_next_character(&mut lexer.reader);
            return create_token(
                get_character_position(&next_character), 
                create_assignment_token_item(create_declaration_assignment_operator())
            );
        }

        return create_token(
            get_character_position(&next_character), 
            create_assignment_token_item(create_initialise_assignment_operator())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_PERIOD) {
        eat_next_character(&mut lexer.reader);
        if is_character(&peek_next_character(&mut lexer.reader), SOURCE_SYMBOL_PERIOD) {
            eat_next_character(&mut lexer.reader);
            return create_token(
                get_character_position(&next_character), 
                create_range_token_item(create_left_inclusive_range())
            );
        }
    }

    if is_character(&next_character, SOURCE_SYMBOL_OPEN_BRACE) {
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_enclosure_token_item(create_open_brace_enclosure())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_OPEN_PAREN) {
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_enclosure_token_item(create_open_parentheses_enclosure())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_CLOSE_BRACE) {
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_enclosure_token_item(create_closed_brace_enclosure())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_CLOSE_PAREN) {
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_enclosure_token_item(create_closed_parentheses_enclosure())
        );
    }

    if is_character(&next_character, SOURCE_SYMBOL_QUOTES) {
        eat_next_character(&mut lexer.reader);
        let string = read_up_until_quotes(lexer);
        eat_next_character(&mut lexer.reader);
        return create_token(
            get_character_position(&next_character), 
            create_string_literal_token_item(string)
        );
    }

    if is_character_alphanumeric(&next_character) {
        let alphanumeric_string = read_up_until_non_alphanumeric(lexer);

        if let Ok(number) = parse_number(&alphanumeric_string) {
            return create_token(
                get_character_position(&next_character), 
                create_number_literal_token_item(number)
            );
        }

        if let Some(built_in_type) = parse_built_in_type(&alphanumeric_string) {
            return create_token(
                get_character_position(&next_character), 
                create_type_token_item(built_in_type)
            );
        }

        if let Some(keyword) = parse_keyword(&alphanumeric_string) {
            return create_token(
                get_character_position(&next_character), 
                create_keyword_token_item(keyword)
            );
        }
    
        return create_token(
            get_character_position(&next_character), 
            create_identifier_token_item(alphanumeric_string)
        );
    }
    
    if is_character_eof(&next_character) {
        return create_token(get_character_position(&next_character), create_eof_token_item())
    }

    create_token(
        get_character_position(&next_character), 
        create_error_token_item(create_unknown_token_error(get_unwrapped_character_value(&next_character)))
    )
}

fn read_up_until_non_alphanumeric(lexer: &mut Lexer) -> String {
    read_characters_up_until(
        &mut lexer.reader, 
        |c| !is_character_alphanumeric(c)
    )
}

fn read_up_until_quotes(lexer: &mut Lexer) -> String {
    read_characters_up_until(
        &mut lexer.reader, 
        |c| is_character(c, SOURCE_SYMBOL_QUOTES)
    )
}

type ParseNumberResult = Result<usize, ParseIntError>;

fn parse_number(from: &str) -> ParseNumberResult {
    from.parse::<usize>()
}
