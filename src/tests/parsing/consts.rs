use crate::parsing::*;
use crate::utilities::*;
use crate::tests::parsing::*;

#[test]
fn parse_const_declaration_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeValue :: 1");
       
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            constant_item(
                string("SomeValue"),
                node(
                    position(13, 1, 14),
                    literal_item(unsigned_int_literal(1)),
                )
            )    
        )
    );
}