use crate::parsing::*;
use super::assert_fail;

#[test]
fn parse_empty_input_parses_correctly() {
    let ast = parse("");
    assert_eq!(1, ast.children.len());
    let child = &ast.children[0];
    assert_eq!(child.item_ref().clone(), AbstractSyntaxNodeItem::Eof);
}

#[test]
fn parse_run_expression_parses_correctly() {
    let ast = parse("#run 1 + 2");
    match ast.children[0].item_ref() {
        AbstractSyntaxNodeItem::Run { expr } => match expr.item_ref() {
            AbstractSyntaxNodeItem::BinaryExpr { op, lhs, rhs } => {
                assert_eq!(&Operator::Add, op);
                assert_eq!(&AbstractSyntaxNodeItem::Literal(Literal::Int(1)), lhs.item_ref());
                assert_eq!(&AbstractSyntaxNodeItem::Literal(Literal::Int(2)), rhs.item_ref());
            },
            _ => assert_fail("BinaryExpr not returned"),
        },
        _ => assert_fail("Run not returned"),
    }
}

#[test]
fn parse_load_expression_parses_correctly() {
    let ast = parse("#load \"test.jai\"");
    
    match ast.children[0].item_ref() {
        AbstractSyntaxNodeItem::Load { file_name } => assert_eq!(file_name, "test.jai"),
        _ => assert_fail("Load not returned"),
    }
}

#[test]
fn parse_const_declaration_parses_correctly() {
    let ast = parse("SomeValue :: 1");

    match ast.children[0].item_ref() {
        AbstractSyntaxNodeItem::Constant { name, value } => {
            assert_eq!(name, "SomeValue");
            assert_eq!(&AbstractSyntaxNodeItem::Literal(Literal::Int(1)), value.item_ref());
        },
        _ => assert_fail("Load not returned"),
    }    
}

#[test]
fn parse_function_declaration_parses_correctly() {
    let ast = parse("SomeFunction :: () {
}");
    
    assert_eq!(
        ast.children[0], 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::FunctionHeader {
                    name: "SomeFunction".to_string(),
                    arguments: vec!(),
                    return_types: vec!(), 
                    body: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                        position: SourceFilePosition { absolute: 19, line: 1, col: 20 }
                    }
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_function_declaration_with_return_type_parses_correctly() {
    let ast = parse("SomeFunction :: () -> void {
}");
    
    assert_eq!(
        ast.children[0], 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::FunctionHeader {
                    name: "SomeFunction".to_string(),
                    arguments: vec!(),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Void))),
                            position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                        }
                    ),
                    body: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                        position: SourceFilePosition { absolute: 27, line: 1, col: 28 }
                    }
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_function_declaration_with_return_types_parses_correctly() {
    let ast = parse("SomeFunction :: () -> SomeType, int {
}");
    
    assert_eq!(
        ast.children[0], 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::FunctionHeader {
                    name: "SomeFunction".to_string(),
                    arguments: vec!(),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::Compound("SomeType".to_string()))),
                            position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                        },
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Int))),
                            position: SourceFilePosition { absolute: 32, line: 1, col: 33 }
                        }
                    ),
                    body: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                        position: SourceFilePosition { absolute: 36, line: 1, col: 37 }
                    }
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_function_declaration_with_arg_parses_correctly() {
    let ast = parse("SomeFunction :: (x: int) {
}");
    
assert_eq!(
    ast.children[0], 
    AbstractSyntaxNode {
        item: Box::new(
            AbstractSyntaxNodeItem::FunctionHeader {
                name: "SomeFunction".to_string(),
                arguments: vec!(
                    AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "x".to_string() , arg_type: Type::BuiltIn(BuiltInType::Int) }),
                        position: SourceFilePosition { absolute: 17, line: 1, col: 18 }
                    }
                ),
                return_types: vec!(), 
                body: AbstractSyntaxNode {
                    item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                    position: SourceFilePosition { absolute: 25, line: 1, col: 26 }
                }
            }
        ),
        position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
    }
);
}

#[test]
fn parse_function_declaration_with_args_and_return_type_parses_correctly() {
    let ast = parse("SomeFunction :: (x: float, y: SomeType) -> void {
}");
    
    assert_eq!(
        ast.children[0], 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::FunctionHeader {
                    name: "SomeFunction".to_string(),
                    arguments: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "x".to_string() , arg_type: Type::BuiltIn(BuiltInType::Float) }),
                            position: SourceFilePosition { absolute: 17, line: 1, col: 18 }
                        },
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "y".to_string() , arg_type: Type::Compound("SomeType".to_string()) }),
                            position: SourceFilePosition { absolute: 27, line: 1, col: 28 }
                        }
                    ),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Void))),
                            position: SourceFilePosition { absolute: 43, line: 1, col: 44 }
                        }
                    ), 
                    body: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                        position: SourceFilePosition { absolute: 48, line: 1, col: 49 }
                    }
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}
