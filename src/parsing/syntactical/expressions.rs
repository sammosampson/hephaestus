use crate::parsing::*;
use crate::errors::*;
use crate::types::*;

pub fn parse_expression(lexer: &mut Lexer, lhs: AbstractSyntaxNode, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    let op = parse_operator(lexer, errors);
    let rhs_node = parse_rhs(lexer, errors);
    create_node(binary_expression_item(op, lhs, rhs_node, unresolved_resolvable_type()), position)
}

pub fn parse_rhs(lexer: &mut Lexer, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position, errors),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position, errors),
        SourceTokenItem::Error(error) => create_error_and_error_node(errors, tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_and_error_node(errors, unimplemented_error(), token.position),
    }
}


pub fn parse_operator(lexer: &mut Lexer, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Operator(op) => create_node(operator_item(op), token.position),
        _ => create_error_and_error_node(errors, expected_operator_error(), token.position),
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
