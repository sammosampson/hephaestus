use crate::parsing::*;

pub fn parse_expression(lexer: &mut Lexer, op: Operator, lhs: AbstractSyntaxNode, position: SourceFilePosition) -> AbstractSyntaxNode {
    eat_next_token(lexer);
    let rhs_node = parse_rhs(lexer);
    create_node(create_expression_item(op, lhs, rhs_node), position)
}

pub fn parse_rhs(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

fn create_expression_item(op: Operator, lhs: AbstractSyntaxNode, rhs: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::BinaryExpr {
        op,
        lhs,
        rhs
    }
}
