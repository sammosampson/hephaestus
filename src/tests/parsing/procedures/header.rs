
use crate::parsing::*;
use crate::typing::*;

#[test]
fn parse_procedure_header_parses_correctly() {
    let content = "SomeProcedure :: () {
}";
    let file_path = "test.hep";

    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        content
    );
    
    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    args: vec!(),
                    return_types: vec!(), 
                    body: CompilationUnitReference::Resolved(units[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_header_with_return_type_parses_correctly() {
    let content = "SomeProcedure :: () -> void {
}";   
    let file_path = "test.hep";

    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        content
    );
    
    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree,  
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    args: vec!(),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(ResolvableType::Resolved(ResolvedTypeId::BuiltInType(BuiltInType::Void)))),
                            position: SourceFilePosition { absolute: 23, line: 1, col: 24 }
                        }
                    ),
                    body: CompilationUnitReference::Resolved(units[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_header_with_return_types_parses_correctly() {
    let content = "SomeProcedure :: () -> SomeType, int {
}";
    let file_path = "test.hep";

    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        content
    );

    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    args: vec!(),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(ResolvableType::UnresolvedNamed("SomeType".to_string()))),
                            position: SourceFilePosition { absolute: 23, line: 1, col: 24 }
                        },
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(ResolvableType::Resolved(ResolvedTypeId::BuiltInType(BuiltInType::Int32)))),
                            position: SourceFilePosition { absolute: 33, line: 1, col: 34 }
                        }
                    ),
                    body: CompilationUnitReference::Resolved(units[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_header_with_arg_parses_correctly() {
    let content = "SomeProcedure :: (x: int) {
}";
    let file_path = "test.hep";

    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        content
    );

    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    args: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(
                                AbstractSyntaxNodeItem::ArgumentDeclaration { 
                                    name: "x".to_string(),
                                    type_id: ResolvableType::Resolved(ResolvedTypeId::BuiltInType(BuiltInType::Int32)),
                                }
                            ),
                            position: SourceFilePosition { absolute: 18, line: 1, col: 19 }
                        }
                    ),
                    return_types: vec!(),
                    body: CompilationUnitReference::Resolved(units[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_procedure_header_with_args_and_return_type_parses_correctly() {
    let content = "SomeProcedure :: (x: float, y: SomeType) -> void {
}";
    let file_path = "test.hep";

    let (actual_file_path, units, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        content
    );

    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::ProcedureHeader {
                    name: "SomeProcedure".to_string(),
                    args: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "x".to_string() , type_id: ResolvableType::Resolved(ResolvedTypeId::BuiltInType(BuiltInType::Float32)) }),
                            position: SourceFilePosition { absolute: 18, line: 1, col: 19 }
                        },
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "y".to_string() , type_id: ResolvableType::UnresolvedNamed("SomeType".to_string()) }),
                            position: SourceFilePosition { absolute: 28, line: 1, col: 29 }
                        }
                    ),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(ResolvableType::Resolved(ResolvedTypeId::BuiltInType(BuiltInType::Void)))),
                            position: SourceFilePosition { absolute: 44, line: 1, col: 45 }
                        }
                    ),
                    body: CompilationUnitReference::Resolved(units[0].id),
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}
