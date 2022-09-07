use crate::parsing::*;

#[test]
fn parse_literal_expression_parses_correctly() {        
    let units = crate::tests::parsing::run_parse_file_return_only_units("main :: () {
        x := 1 + 2;
    }");

    dbg!(&units[0].tree);
    dbg!(&units[1].tree);

    assert_eq!(units.len(), 2);
    assert_eq!(
        units[0].tree, 
        AbstractSyntaxNode {
            position: SourceFilePosition { absolute: 11, line: 1, col: 12 },
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureBody(vec!(
                    AbstractSyntaxNode {
                        position: SourceFilePosition { absolute: 21, line: 2, col: 9 },
                        item: Box::new(
                            AbstractSyntaxNodeItem::Assignment {            
                                name: "x".to_string(), 
                                value:
                                    AbstractSyntaxNode {
                                        position: SourceFilePosition { absolute: 26, line: 2, col: 14 },
                                        item: Box::new(
                                            AbstractSyntaxNodeItem::BinaryExpr {
                                                op: Operator::Add,
                                                lhs: AbstractSyntaxNode {
                                                    item: Box::new(AbstractSyntaxNodeItem::Literal(Literal::Int(1))),
                                                    position: SourceFilePosition { absolute: 26, line: 2, col: 14 }
                                                },
                                                rhs: AbstractSyntaxNode {
                                                    item: Box::new(AbstractSyntaxNodeItem::Literal(Literal::Int(2))),
                                                    position: SourceFilePosition { absolute: 30, line: 2, col: 18 }
                                                }
                                            }
                                        ),
                                    }
                            }
                        )
                    }
                ))
            )
        }
    )
}

#[test]
fn parse_variable_expression_parses_correctly() {        
    let units = crate::tests::parsing::run_parse_file_return_only_units("main :: (a: int, b: int) {
    x := a + b;
}");    
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[0].tree, 
        AbstractSyntaxNode {
            position: SourceFilePosition { absolute: 25, line: 1, col: 26 },
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureBody(vec!(
                    AbstractSyntaxNode {
                        position: SourceFilePosition { absolute: 31, line: 2, col: 5 },
                        item: Box::new(
                            AbstractSyntaxNodeItem::Assignment {            
                                name: "x".to_string(), 
                                value:
                                    AbstractSyntaxNode {
                                        position: SourceFilePosition { absolute: 36, line: 2, col: 10 },
                                        item: Box::new(
                                            AbstractSyntaxNodeItem::BinaryExpr {
                                                op: Operator::Add,
                                                lhs: AbstractSyntaxNode {
                                                    item: Box::new(AbstractSyntaxNodeItem::Identifier{ name: "a".to_string() }),
                                                    position: SourceFilePosition { absolute: 36, line: 2, col: 10 }
                                                },
                                                rhs: AbstractSyntaxNode {
                                                    item: Box::new(AbstractSyntaxNodeItem::Identifier { name: "b".to_string() }),
                                                    position: SourceFilePosition { absolute: 40, line: 2, col: 14 }
                                                }
                                            }
                                        ),
                                    }
                            }
                        )
                    }
                ))
            )
        }
    )
}