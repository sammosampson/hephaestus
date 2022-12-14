use crate::parsing::*;
use crate::types::*;

pub fn parse_expression(lexer: &mut Lexer, lhs: AbstractSyntaxNode, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    let op = parse_operator(lexer)?;
    let rhs_node = parse_rhs(lexer)?;
    Ok(create_node(binary_expression_item(op, lhs, rhs_node, unresolved_resolvable_type()), position))
}

pub fn parse_rhs(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => Err(create_error(tokenisation_error(error), token.position)),
        SourceTokenItem::Eof => Ok(create_node(create_eof_item(), token.position)),
        _ => Err(create_error(unimplemented_error(), token.position)),
    }
}


pub fn parse_operator(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Operator(op) => Ok(create_node(operator_item(op), token.position)),
        _ => Err(create_error(expected_operator_error(), token.position)),
    }
}

pub fn binary_expression_item(
    op: AbstractSyntaxNode,
    lhs: AbstractSyntaxNode,
    rhs: AbstractSyntaxNode,
    type_id: ResolvableType
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::BinaryExpr {
        op,
        lhs,
        rhs,
        expression_type: type_id
    }
}

pub fn operator_item(op: Operator) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Operator(op)
}
