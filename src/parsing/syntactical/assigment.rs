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
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

pub fn assignment_item(name: String, value: AbstractSyntaxNode, type_id: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Assignment { name, value, assignment_type: type_id }
}