use crate::parsing::*;
use crate::tests::parsing::*;

#[test]
fn parse_run_directive_parses_correctly() {        
    let units = crate::tests::parsing::run_parse_file_return_only_units("#run 1 + 2");
    
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            run_directive_item(
                node(
                    position(5, 1, 6),
                    binary_expression_item(
                        node(
                            position(7, 1, 8),
                            operator_item(add_operator())
                        ),
                        node(
                            position(5, 1, 6),
                            literal_item(int_literal(1)),
                        ),
                        node(
                            position(9, 1, 10),
                            literal_item(int_literal(2)),
                        )
                    )
                )  
            )
        )
    )
}

#[test]
fn parse_load_directive_parses_correctly() {
    let units = crate::tests::parsing::run_parse_file_return_only_units("#load \"test.jai\"");
       
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            load_directive_item("test.jai".to_string())
        )
    );
}
