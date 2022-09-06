use crate::parsing::*;
use crate::typing::*;

#[test]
fn parse_procedure_body_parses_correctly() {
    let file_path = "test.hep";
    let content = &"SomeProcedure :: () {
    a := 1;
    SomeOtherProcedure(a);
}";
    
    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        content
    );
       
    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(
        units[0].tree, 
        AbstractSyntaxNode {
            position: SourceFilePosition { absolute: 20, line: 1, col: 21 },
            item: Box::new(AbstractSyntaxNodeItem::ProcedureBody(vec!(
                AbstractSyntaxNode {
                    position: SourceFilePosition { absolute: 26, line: 2, col: 5 },
                    item: Box::new(
                        AbstractSyntaxNodeItem::Assignment {
                            name: "a".to_string(),
                            value: AbstractSyntaxNode {
                                position: SourceFilePosition { absolute: 31, line: 2, col: 10 },
                                item: Box::new(
                                    AbstractSyntaxNodeItem::Literal(Literal::Int(1))
                                ),
                            },
                        }
                    ),
                },                        
                AbstractSyntaxNode {                    
                    position: SourceFilePosition { absolute: 38, line: 3, col: 5 },
                    item: Box::new(
                        AbstractSyntaxNodeItem::ProcedureCall {
                            name: "SomeOtherProcedure".to_string(),
                            args: vec!(
                                AbstractSyntaxNode {
                                    position: SourceFilePosition { absolute: 57, line: 3, col: 24 },
                                    item: Box::new(
                                        AbstractSyntaxNodeItem::Argument { 
                                            expr: AbstractSyntaxNode {
                                                position: SourceFilePosition { absolute: 57, line: 3, col: 24 },
                                                item: Box::new(
                                                    AbstractSyntaxNodeItem::Identifier("a".to_string())
                                                ),
                                            },
                                            type_id: ResolvableType::Unresolved
                                        }
                                    )
                                }
                            ),
                            type_id: ResolvableType::Unresolved
                        }
                    )
                }                        
            )))
        }
    );
}