use crate::parsing::*;

#[test]
fn parse_run_directive_parses_correctly() {    
    let file_path = "test.hep";
    
    let (actual_file_path, units) = crate::tests::parsing::run_parse_file(
        file_path, 
        "#run 1 + 2"
    );
    
    assert_eq!(actual_file_path, file_path.to_string());
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
fn parse_load_directive_parses_correctly() {
    let file_path = "test.hep";
    
    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        "#load \"test.jai\""
    );
       
    assert_eq!(actual_file_path, file_path.to_string());
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
