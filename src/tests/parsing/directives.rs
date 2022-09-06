use crate::parsing::*;

#[test]
fn parse_run_directive_parses_correctly() {        
    let units = crate::tests::parsing::run_parse_file_return_only_units("#run 1 + 2");
    
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
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
fn parse_run_directive_with_procedure_call_parses_correctly() {        
    let units = crate::tests::parsing::run_parse_file_return_only_units("main :: (a: int, b: int) {
    x := a + b;
}
#run main(1, 2)");
    
    assert_eq!(units.len(), 3);
    dbg!(&units[0].tree);
}

#[test]
fn parse_load_directive_parses_correctly() {
    let units = crate::tests::parsing::run_parse_file_return_only_units("#load \"test.jai\"");
       
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
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
