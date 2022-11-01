use crate::parsing::*;
use crate::compilation::*;
use crate::types::*;

pub fn parse_inferred_value_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    parse_value_assignment(name, lexer, position, unresolved_resolvable_type(), errors)
}

fn parse_value_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition, resolvable_type: ResolvableType, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    create_node(variable_declaration_item(name, parse_value_assignment_value(lexer, errors), resolvable_type), position)
}

pub fn parse_initialise_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    if let Some(resolvable_type) = try_parse_type(lexer) {
        eat_next_token(lexer);
        if is_assign_value_assignment(&peek_next_token(lexer).item) {
            eat_next_token(lexer);
            return parse_value_assignment(name, lexer, position, resolvable_type, errors);
        }        
        return create_error_and_error_node(errors, expected_assign_value_assignment_error(), get_next_token(lexer).position);
    }
    create_error_and_error_node(errors, expected_type_error(), get_next_token(lexer).position)
}

pub fn parse_value_assignment_value(lexer: &mut Lexer, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position, errors),
        SourceTokenItem::Identifier(identifier) => parse_identifier(identifier, lexer, token.position, errors),
        SourceTokenItem::Error(error) => create_error_and_error_node(errors, tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        SourceTokenItem::Keyword(keyword) => parse_value_assignment_keyword(keyword, lexer, token.position, errors),
        
        _ => create_error_and_error_node(errors, unimplemented_error(), token.position),
    }
}

fn parse_value_assignment_keyword(keyword: Keyword, lexer: &mut Lexer, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    match keyword {
        Keyword::Cast => parse_value_assignment_cast(lexer, position, errors),
        Keyword::Null => create_node(null_item(), position),
        _ => create_error_and_error_node(errors, unimplemented_error(), position),
    }
}

fn parse_value_assignment_cast(lexer: &mut Lexer, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    if !is_open_paren(&peek_next_token(lexer).item) {
        return create_error_and_error_node(errors, expected_open_paren_error(), get_next_token(lexer).position)
    }
    
    eat_next_token(lexer);
    
    if let Some(cast_type) = try_parse_type(lexer) {
        eat_next_token(lexer);

        if !is_close_paren(&peek_next_token(lexer).item) {
            return create_error_and_error_node(errors, expected_close_paren_error(), get_next_token(lexer).position)
        }

        eat_next_token(lexer);
        return create_node(cast_item(cast_type, parse_value_assignment_value(lexer, errors)), position);
    }   

    create_error_and_error_node(errors, expected_type_error(), get_next_token(lexer).position)
}

pub fn variable_declaration_item(name: String, value: AbstractSyntaxNode, type_id: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::VariableDeclaration { name, value, variable_type: type_id }
}

pub fn cast_item(cast_type: ResolvableType, expr: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Cast { cast_type, expr }
}