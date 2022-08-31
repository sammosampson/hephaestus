use crate::parsing::*;

pub fn parse_expression(lexer: &mut Lexer, op: Operator, lhs: AbstractSyntaxNode, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    eat_next_token(lexer);
    let rhs_node = parse_next_node(lexer, units);
    create_node(create_expression_item(op, lhs, rhs_node), position)
}

fn create_expression_item(op: Operator, lhs: AbstractSyntaxNode, rhs: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::BinaryExpr {
        op,
        lhs,
        rhs
    }
}
