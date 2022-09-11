
use crate::{
    intermediate_representation::*,
    tests::intermediate_representation::*
};

#[test]
fn byte_code_for_variable_int_literal_assignment_generates_correctly() {
    let code = compile_source_and_get_intemediate_representation("SomeProcedure :: () {
    a := 1;
}"
    );
    
    assert_eq!(code.len(), 2); 
    assert_eq!(
        code[1].byte_code, 
        vec!(
            ByteCodeInstruction::LII,
            ByteCodeInstruction::R(0),
            ByteCodeInstruction::RVAL(1),
        )
    );
}