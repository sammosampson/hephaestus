use crate::parsing::*;

#[test]
fn parse_procedure_call_parses_correctly() {
    let ast = parse("#run SomeProcedure()");
    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::Run {
                    expr: AbstractSyntaxNode {
                        item: Box::new(
                            AbstractSyntaxNodeItem::ProcedureCall {
                                name: "SomeProcedure".to_string(),
                                arguments: vec!(),
                                arg_type: Type::Unknown
                            }
                        ),
                        position: SourceFilePosition { absolute: 5, line: 1, col: 6 }
                    }                        
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_call_with_arg_parses_correctly() {
    let ast = parse("#run SomeProcedure(a, b)");
    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::Run {
                    expr: AbstractSyntaxNode {
                        item: Box::new(
                            AbstractSyntaxNodeItem::ProcedureCall {
                                name: "SomeProcedure".to_string(),
                                arguments: vec!(
                                    AbstractSyntaxNode {
                                        item: Box::new(
                                            AbstractSyntaxNodeItem::Argument { 
                                                expr: AbstractSyntaxNode {
                                                    item: Box::new(
                                                        AbstractSyntaxNodeItem::Identifier("a".to_string())
                                                    ),
                                                    position: SourceFilePosition { absolute: 19, line: 1, col: 20 }
                                                },
                                                arg_type: Type::Unknown
                                            }
                                        ),
                                        position: SourceFilePosition { absolute: 19, line: 1, col: 20 }
                                    },                                    
                                    AbstractSyntaxNode {
                                        item: Box::new(
                                            AbstractSyntaxNodeItem::Argument { 
                                                expr: AbstractSyntaxNode {
                                                    item: Box::new(
                                                        AbstractSyntaxNodeItem::Identifier("b".to_string())
                                                    ),
                                                    position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                                                },
                                                arg_type: Type::Unknown
                                            }
                                        ),
                                        position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                                    }
                                ),
                                arg_type: Type::Unknown
                            }
                        ),
                        position: SourceFilePosition { absolute: 5, line: 1, col: 6 }
                    }                        
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}
