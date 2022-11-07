use crate::{parsing::*, strings::*};

pub fn parse_directive(directive: Directive, lexer: &mut Lexer, position: SourceFilePosition)-> AbstractSyntaxNodeResult {
    match directive {
        Directive::Run => parse_run_directive(lexer, position),
        Directive::Load => parse_load_directive(lexer, position),
        _ => Err(create_error(unexpected_directive_error(), position)),
    }
}

pub fn parse_const_directive(directive: Directive, lexer: &mut Lexer, position: SourceFilePosition)-> AbstractSyntaxNodeResult {
    match directive {
        Directive::ForeignSystemLibrary => parse_foreign_system_library_directive(lexer, position),
        _ => Err(create_error(unexpected_directive_error(), position)),
    }
}

fn parse_run_directive(lexer: &mut Lexer, position: SourceFilePosition)-> AbstractSyntaxNodeResult {
    Ok(create_node(run_directive_item(parse_run_directive_expr(lexer)?), position))
}

pub fn parse_run_directive_expr(lexer: &mut Lexer)-> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => Err(create_error(tokenisation_error(error), token.position)),
        SourceTokenItem::Eof => Ok(create_node(create_eof_item(), token.position)),
        _ => Err(create_error(unimplemented_error(), token.position)),
    }
}

fn parse_load_directive(lexer: &mut Lexer, position: SourceFilePosition)-> AbstractSyntaxNodeResult {
    Ok(create_node(load_directive_item(parse_load_file(lexer)?), position))
}

pub fn parse_load_file(lexer: &mut Lexer)-> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);
    if let Some(literal) = try_get_string_literal(&token.item) {
        return parse_ending_string_literal(lexer, literal, token.position);
    }
    Err(create_error(expected_file_name_error(), token.position))
}

fn parse_foreign_system_library_directive(lexer: &mut Lexer, position: SourceFilePosition)-> AbstractSyntaxNodeResult {
    Ok(create_node(foreign_system_library_directive_item(parse_foreign_system_library(lexer)?), position))
}

pub fn parse_foreign_system_library(lexer: &mut Lexer)-> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);
    if let Some(literal) = try_get_string_literal(&token.item) {
        return parse_ending_string_literal(lexer, literal, token.position);
    }
    Err(create_error(expected_library_name_error(), token.position))
}

fn parse_ending_string_literal(lexer: &mut Lexer, literal: ByteString, position: SourceFilePosition)-> AbstractSyntaxNodeResult {
    if is_line_terminiator(&peek_next_token(lexer).item) {
        eat_next_token(lexer)
    }
    Ok(create_node(literal_item(unresolved_resolvable_literal(unresolved_string_literal(literal))), position))
}


pub fn run_directive_item(expr: AbstractSyntaxNode)-> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Run { expr }
}

pub fn load_directive_item(file: AbstractSyntaxNode)-> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Load { file }
}

pub fn foreign_system_library_directive_item(library: AbstractSyntaxNode)-> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ForeignSystemLibrary { library }
}