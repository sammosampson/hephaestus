use crate::parsing::*;
use crate::errors::*;

pub fn parse_literal(literal: UnresolvedLiteral, lexer: &mut Lexer, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    let node = create_node(literal_item(unresolved_resolvable_literal(literal)), position);
    
    if is_operator(&peek_next_token(lexer).item) {
        return parse_expression(lexer, node, position, errors);
    }

    node
}

pub fn literal_item(literal: ResolvableLiteral) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Literal(literal)
}