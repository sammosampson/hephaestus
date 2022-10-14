use crate::parsing::*;
use crate::typing::*;

pub fn parse_inferred_value_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    parse_value_assignment(name, lexer, position, unresolved_resolvable_type())
}

fn parse_value_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition, resolvable_type: ResolvableType) -> AbstractSyntaxNode {
    create_node(assignment_item(name, parse_value_assignment_value(lexer), resolvable_type), position)
}

pub fn parse_initialise_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if let Some(resolvable_type) = try_parse_type(lexer) {
        eat_next_token(lexer);
        if is_assign_value_assignment(&peek_next_token(lexer).item) {
            eat_next_token(lexer);
            return parse_value_assignment(name, lexer, position, resolvable_type);
        }        
        return create_error_node(expected_assign_value_assignment_error(), get_next_token(lexer).position);
    }
    create_error_node(expected_type_error(), get_next_token(lexer).position)
}

pub fn parse_value_assignment_value(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Identifier(identifier) => parse_identifier(identifier, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        SourceTokenItem::Keyword(keyword) => parse_value_assignment_keyword(keyword, lexer, token.position),
        
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

fn parse_value_assignment_keyword(keyword: Keyword, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    match keyword {
        Keyword::Cast => parse_value_assignment_cast(lexer, position),
        Keyword::Null => create_node(null_item(), position),
        _ => create_error_node(unimplemented_error(), position),
    }
}

fn parse_value_assignment_cast(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if !is_open_paren(&peek_next_token(lexer).item) {
        return create_error_node(expected_open_paren_error(), get_next_token(lexer).position)
    }
    
    eat_next_token(lexer);
    
    if let Some(cast_type) = try_parse_type(lexer) {
        eat_next_token(lexer);

        if !is_close_paren(&peek_next_token(lexer).item) {
            return create_error_node(expected_close_paren_error(), get_next_token(lexer).position)
        }

        eat_next_token(lexer);
        return create_node(cast_item(cast_type, parse_value_assignment_value(lexer)), position);
    }   

    create_error_node(expected_type_error(), get_next_token(lexer).position)
}

pub fn assignment_item(name: String, value: AbstractSyntaxNode, type_id: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Assignment { name, value, assignment_type: type_id }
}

pub fn cast_item(cast_type: ResolvableType, expr: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Cast { cast_type, expr }
}