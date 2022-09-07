use crate::parsing::*;

pub fn parse_literal(literal: Literal, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let node = create_node(literal_item(literal), position);
    
    if is_operator(&peek_next_token(lexer).item) {
        return parse_expression(lexer, node, position);
    }

    node
}

pub fn literal_item(literal: Literal) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Literal(literal)
}