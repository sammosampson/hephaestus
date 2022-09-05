use crate::parsing::*;

pub fn parse_literal(literal: Literal, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let node = create_node(create_literal_item(literal), position);
    
    if let SourceTokenItem::Operator(op) = peek_next_token(lexer).item {
        return parse_expression(lexer, op, node, position);
    }

    node
}

fn create_literal_item(literal: Literal) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Literal(literal)
}