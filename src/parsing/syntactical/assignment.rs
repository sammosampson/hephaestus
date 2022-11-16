use crate::parsing::*;
use crate::types::*;

pub fn parse_inferred_value_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    parse_value_assignment(name, lexer, position, unresolved_resolvable_type())
}

fn parse_value_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition, resolvable_type: ResolvableType) -> AbstractSyntaxNodeResult {
    Ok(create_node(variable_declaration_item(name, parse_value_assignment_value(lexer)?, resolvable_type), position))
}

pub fn parse_initialise_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    if let Some(resolvable_type) = try_parse_type(lexer) {
        eat_next_token(lexer);
        if is_assign_value_assignment(&peek_next_token(lexer).item) {
            eat_next_token(lexer);
            return parse_value_assignment(name, lexer, position, resolvable_type);
        }        
        return Err(create_error(expected_assign_value_assignment_error(), get_next_token(lexer).position));
    }
    Err(create_error(expected_type_error(), get_next_token(lexer).position))
}

pub fn parse_value_assignment_value(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Identifier(identifier) => parse_identifier(identifier, lexer, token.position),
        SourceTokenItem::Error(error) => Err(create_error(tokenisation_error(error), token.position)),
        SourceTokenItem::Eof => Ok(create_node(create_eof_item(), token.position)),
        SourceTokenItem::Keyword(keyword) => parse_value_assignment_keyword(keyword, lexer, token.position),
        _ => Err(create_error(unimplemented_error(), token.position)),
    }
}

fn parse_value_assignment_keyword(keyword: Keyword, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    match keyword {
        Keyword::Cast => parse_value_assignment_cast(lexer, position),
        Keyword::Null => Ok(create_node(null_item(), position)),
        _ => Err(create_error(unimplemented_error(), position)),
    }
}

fn parse_value_assignment_cast(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    if !is_open_paren(&peek_next_token(lexer).item) {
        return Err(create_error(expected_open_paren_error(), get_next_token(lexer).position))
    }
    
    eat_next_token(lexer);
    
    if let Some(cast_type) = try_parse_type(lexer) {
        eat_next_token(lexer);

        if !is_close_paren(&peek_next_token(lexer).item) {
            return Err(create_error(expected_close_paren_error(), get_next_token(lexer).position))
        }

        eat_next_token(lexer);
        return Ok(create_node(cast_item(cast_type, parse_value_assignment_value(lexer)?), position));
    }   

    Err(create_error(expected_type_error(), get_next_token(lexer).position))
}

pub fn variable_declaration_item(name: String, value: AbstractSyntaxNode, type_id: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::VariableDeclaration { name, value, variable_type: type_id }
}

pub fn cast_item(cast_type: ResolvableType, expr: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Cast { cast_type, expr }
}