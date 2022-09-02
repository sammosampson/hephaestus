
use crate::parsing::*;

#[test]
fn parse_procedure_header_parses_correctly() {
    let ast = parse("SomeProcedure :: () {
}");
    
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(AbstractSyntaxNodeItem::ProcedureBody(vec!())),
            position: SourceFilePosition { absolute: 20, line: 1, col: 21 }
        }
    );

    assert_eq!(
        ast[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    arguments: vec!(),
                    return_types: vec!(), 
                    body: CompilationUnitReference::Resolved(ast[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_header_with_return_type_parses_correctly() {
    let ast = parse("SomeProcedure :: () -> void {
}");
       
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(AbstractSyntaxNodeItem::ProcedureBody(vec!())),
            position: SourceFilePosition { absolute: 28, line: 1, col: 29 }
        }
    );
    assert_eq!(
        ast[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    arguments: vec!(),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Void))),
                            position: SourceFilePosition { absolute: 23, line: 1, col: 24 }
                        }
                    ),
                    body: CompilationUnitReference::Resolved(ast[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_header_with_return_types_parses_correctly() {
    let ast = parse("SomeProcedure :: () -> SomeType, int {
}");
    
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(AbstractSyntaxNodeItem::ProcedureBody(vec!())),
            position: SourceFilePosition { absolute: 37, line: 1, col: 38 }
        }
    );
    assert_eq!(
        ast[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    arguments: vec!(),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::Compound("SomeType".to_string()))),
                            position: SourceFilePosition { absolute: 23, line: 1, col: 24 }
                        },
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Int))),
                            position: SourceFilePosition { absolute: 33, line: 1, col: 34 }
                        }
                    ),
                    body: CompilationUnitReference::Resolved(ast[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_header_with_arg_parses_correctly() {
    let ast = parse("SomeProcedure :: (x: int) {
}");
    
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(AbstractSyntaxNodeItem::ProcedureBody(vec!())),
            position: SourceFilePosition { absolute: 26, line: 1, col: 27 }
        }
    );
    assert_eq!(
        ast[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    arguments: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "x".to_string() , arg_type: Type::BuiltIn(BuiltInType::Int) }),
                            position: SourceFilePosition { absolute: 18, line: 1, col: 19 }
                        }
                    ),
                    return_types: vec!(),
                    body: CompilationUnitReference::Resolved(ast[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_header_with_args_and_return_type_parses_correctly() {
    let ast = parse("SomeProcedure :: (x: float, y: SomeType) -> void {
}");
    
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(AbstractSyntaxNodeItem::ProcedureBody(vec!())),
            position: SourceFilePosition { absolute: 49, line: 1, col: 50 }
        }
    );
    assert_eq!(
        ast[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    arguments: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "x".to_string() , arg_type: Type::BuiltIn(BuiltInType::Float) }),
                            position: SourceFilePosition { absolute: 18, line: 1, col: 19 }
                        },
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "y".to_string() , arg_type: Type::Compound("SomeType".to_string()) }),
                            position: SourceFilePosition { absolute: 28, line: 1, col: 29 }
                        }
                    ),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Void))),
                            position: SourceFilePosition { absolute: 44, line: 1, col: 45 }
                        }
                    ),
                    body: CompilationUnitReference::Resolved(ast[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}
