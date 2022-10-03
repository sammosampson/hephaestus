use crate::parsing::*;

pub fn parse_literal(literal: UnresolvedLiteral, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let node = create_node(literal_item(unresolved_resolvable_literal(literal)), position);
    
    if is_operator(&peek_next_token(lexer).item) {
        return parse_expression(lexer, node, position);
    }

    node
}

pub fn literal_item(literal: ResolvableLiteral) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Literal(literal)
}