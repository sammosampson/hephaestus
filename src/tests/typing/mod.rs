use crate::parsing::*;
use crate::typing::*;

#[test]
fn typing_procedure_header_returns_correct_types() {
    let content = "SomeProcedure :: () {
    }";

    let file_path = "test.hep";

    let (_actual_file_path, mut ast, ..) = crate::tests::parsing::run_parse_file(
        file_path, 
        content
    );

    let mut proc_header = ast.pop().unwrap();
    let proc_header_id = proc_header.id;
    let _proc_body = ast.pop().unwrap();
    let types = perform_typing(&mut proc_header, |c| { ResolvedTypeId::BuiltInType(BuiltInType::Float32) });

    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, ResolvedTypeId::UserDefined(proc_header_id));
    assert_eq!(types[0].name, "SomeProcedure".to_string());
    assert_eq!(types[0].item, TypeItem::ProcedureDefinition { arg_types: vec!(), return_types: vec!()});
    assert_eq!(types[0].size, TypeSize::Unresolved);
}


#[test]
fn typing_procedure_header_with_args_returns_correct_types() {
    let mut ast = parse("SomeProcedure :: (a: int, b: float) -> float, int {
}");
    let mut proc_header = ast.pop().unwrap();
    let types = perform_typing(&mut proc_header, |c| { ResolvedTypeId::BuiltInType(BuiltInType::Float32) });

    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, ResolvedTypeId::UserDefined(proc_header.id));
    assert_eq!(types[0].name, "SomeProcedure".to_string());
    assert_eq!(types[0].item, TypeItem::ProcedureDefinition { 
        arg_types: vec!(
            ResolvedTypeId::BuiltInType(BuiltInType::Int32),
            ResolvedTypeId::BuiltInType(BuiltInType::Float32),
        ), 
        return_types: vec!(
            ResolvedTypeId::BuiltInType(BuiltInType::Float32),
            ResolvedTypeId::BuiltInType(BuiltInType::Int32),
        )
    });
    assert_eq!(types[0].size, TypeSize::Unresolved);
}


#[test]
fn typing_procedure_body_waits_for_external_procedure() {
    let mut ast = parse("SomeProcedure :: () {
    SomeOtherProcedure();
}");

    let mut proc_header = ast.pop().unwrap();
    let _proc_header_id = proc_header.id;
    let _types = perform_typing(&mut proc_header, |c| { ResolvedTypeId::BuiltInType(BuiltInType::Float32) });
}

