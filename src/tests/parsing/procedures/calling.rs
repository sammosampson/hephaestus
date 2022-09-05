use crate::parsing::*;
use crate::typing::*;

#[test]
fn parse_procedure_call_parses_correctly() {
    let file_path = "test.hep";
    
    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        "#run SomeProcedure()"
    );
       
    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::Run {
                    expr: AbstractSyntaxNode {
                        item: Box::new(
                            AbstractSyntaxNodeItem::ProcedureCall {
                                name: "SomeProcedure".to_string(),
                                args: vec!(),
                                arg_type: ResolvableType::Unresolved
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
    let content = "#run SomeProcedure(a, b)";
    let file_path = "test.hep";
    
    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        content
    );
       
    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::Run {
                    expr: AbstractSyntaxNode {
                        item: Box::new(
                            AbstractSyntaxNodeItem::ProcedureCall {
                                name: "SomeProcedure".to_string(),
                                args: vec!(
                                    AbstractSyntaxNode {
                                        item: Box::new(
                                            AbstractSyntaxNodeItem::Argument { 
                                                expr: AbstractSyntaxNode {
                                                    item: Box::new(
                                                        AbstractSyntaxNodeItem::Identifier("a".to_string())
                                                    ),
                                                    position: SourceFilePosition { absolute: 19, line: 1, col: 20 }
                                                },
                                                arg_type: ResolvableType::Unresolved
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
                                                arg_type: ResolvableType::Unresolved
                                            }
                                        ),
                                        position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                                    }
                                ),
                                arg_type: ResolvableType::Unresolved
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
