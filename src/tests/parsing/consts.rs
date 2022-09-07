use crate::parsing::*;

#[test]
fn parse_const_declaration_parses_correctly() {
    let units = crate::tests::parsing::run_parse_file_return_only_units(
        "SomeValue :: 1"
    );
       
    assert_eq!(
        units[0].tree, 
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