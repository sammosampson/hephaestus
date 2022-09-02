use crate::parsing::*;

#[test]
fn parse_const_declaration_parses_correctly() {
    let ast = parse("SomeValue :: 1");
    
    assert_eq!(
        ast[0].tree, 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::Constant {
                    name: "SomeValue".to_string(),
                    value: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::Literal(Literal::Int(1))),
                        position: SourceFilePosition { absolute: 13, line: 1, col: 14 }            
                    }            
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}