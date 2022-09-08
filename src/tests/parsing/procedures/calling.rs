use crate::parsing::*;
use crate::typing::*;
use crate::tests::parsing::*;

#[test]
fn parse_procedure_call_parses_correctly() {
    let units = run_parse_file_return_only_units("#run SomeProcedure()");
       
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            AbstractSyntaxNodeItem::Run {
                expr: node(
                    position(5, 1, 6),
                    procedure_call_item(
                        string("SomeProcedure"),
                        vec!(),
                        unresolved_resolvable_type(),
                        vec!()
                    )
                ),                        
            }      
        )
    );
}

#[test]
fn parse_procedure_call_with_arg_parses_correctly() {
    let units= run_parse_file_return_only_units("#run SomeProcedure(a, b)");
       
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            run_directive_item(
                node(
                    position(5, 1, 6),
                    procedure_call_item(
                        string("SomeProcedure"),
                        vec!(
                            node(
                                position(19, 1, 20),
                                arg_item(
                                    node(                                                    
                                        position(19, 1, 20),
                                        identifier_item(string("a"))
                                    ),
                                    unresolved_resolvable_type()
                                )
                            ),
                            node(
                                position(22, 1, 23),
                                arg_item( 
                                    node(
                                        position(22, 1, 23),
                                        identifier_item(string("b"))
                                    ),
                                    unresolved_resolvable_type()
                                )
                            ),
                        ),
                        unresolved_resolvable_type(),
                        vec!()
                    )
                )                        
            )
        )
    );
}
