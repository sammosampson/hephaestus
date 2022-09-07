use crate::parsing::*;
use crate::typing::*;

#[test]
fn parse_procedure_call_parses_correctly() {
    let units = crate::tests::parsing::run_parse_file_return_only_units("#run SomeProcedure()");
       
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
                                type_id: ResolvableType::Unresolved
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
    let units= crate::tests::parsing::run_parse_file_return_only_units(content);
       
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
                                                        AbstractSyntaxNodeItem::Identifier { name: "a".to_string() }
                                                    ),
                                                    position: SourceFilePosition { absolute: 19, line: 1, col: 20 }
                                                },
                                                type_id: ResolvableType::Unresolved
                                            }
                                        ),
                                        position: SourceFilePosition { absolute: 19, line: 1, col: 20 }
                                    },                                    
                                    AbstractSyntaxNode {
                                        item: Box::new(
                                            AbstractSyntaxNodeItem::Argument { 
                                                expr: AbstractSyntaxNode {
                                                    item: Box::new(
                                                        AbstractSyntaxNodeItem::Identifier{ name: "b".to_string() }
                                                    ),
                                                    position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                                                },
                                                type_id: ResolvableType::Unresolved
                                            }
                                        ),
                                        position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                                    }
                                ),
                                type_id: ResolvableType::Unresolved
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
