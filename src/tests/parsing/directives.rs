use crate::parsing::*;

#[test]
fn parse_run_directive_parses_correctly() {
    let ast = parse("#run 1 + 2");
    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::Run {
                    expr: 
                        AbstractSyntaxNode {
                            item: Box::new(
                                AbstractSyntaxNodeItem::BinaryExpr {
                                    op: Operator::Add,
                                    lhs: AbstractSyntaxNode {
                                        item: Box::new(AbstractSyntaxNodeItem::Literal(Literal::Int(1))),
                                        position: SourceFilePosition { absolute: 5, line: 1, col: 6 }
                                    },
                                    rhs: AbstractSyntaxNode {
                                        item: Box::new(AbstractSyntaxNodeItem::Literal(Literal::Int(2))),
                                        position: SourceFilePosition { absolute: 9, line: 1, col: 10 }
                                    }
                                }
                            ),
                            position: SourceFilePosition { absolute: 5, line: 1, col: 6 }
                        }
                    }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    )
}

#[test]
fn parse_load_directive_parses_correctly() {
    let ast = parse("#load \"test.jai\"");
    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::Load {
                    file_name: "test.jai".to_string()
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}
