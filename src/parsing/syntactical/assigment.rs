use crate::parsing::*;
use crate::typing::*;

pub fn parse_value_assignment(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(assignment_item(name, parse_value_assignment_value(lexer), unresolved_resolvable_type()), position)
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