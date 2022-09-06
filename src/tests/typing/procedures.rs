use crate::typing::*;
use crate::parsing::*;

#[test]
fn typing_procedure_header_returns_correct_types() {
    let file_path = "test.hep";
    let content = "SomeProcedure :: () {
    }";

    let mut units = crate::tests::parsing::run_parse_file_return_only_units(
        file_path, 
        content
    );

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;
    
    let (types, _unit) = crate::tests::typing::run_typing_on_unit(
        crate::tests::typing::start_type_repository_actor(), 
        proc_header
    );
    
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, ResolvedTypeId::UserDefined(proc_header_id));
    assert_eq!(types[0].name, "SomeProcedure".to_string());
    assert_eq!(types[0].item, TypeItem::ProcedureDefinition { arg_types: vec!(), return_types: vec!()});
    assert_eq!(types[0].size, TypeSize::Unresolved);
}


#[test]
fn typing_procedure_header_with_args_returns_correct_types() {
    let file_path = "test.hep";
    let content = "SomeProcedure :: (a: int, b: float) -> float, int {
}";
    
    let mut units = crate::tests::parsing::run_parse_file_return_only_units(
        file_path, 
        content
    );

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;

    let (types, _unit) = crate::tests::typing::run_typing_on_unit(
        crate::tests::typing::start_type_repository_actor(), 
        proc_header
    );

    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, ResolvedTypeId::UserDefined(proc_header_id));
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
    let file_path = "test.hep";
    let content = "SomeProcedure :: () {
    SomeOtherProcedure();
}";

    let mut units = crate::tests::parsing::run_parse_file_return_only_units(
        file_path, 
        content
    );

    let typing_repository = crate::tests::typing::start_type_repository_actor();

    let other_proc_type_id = create_compilation_unit_id();

    let other_proc_type = create_type(
        other_proc_type_id, 
        "SomeOtherProcedure".to_string(),
        create_procedure_defnition_type_item(vec!(), vec!())
    );

    crate::tests::typing::add_resolved_type(&typing_repository, other_proc_type);

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();

    let (types, unit) = crate::tests::typing::run_typing_on_unit(
        typing_repository, 
        proc_body
    );

    assert_eq!(types.len(), 0);
    assert_eq!(
        unit.tree, 
        AbstractSyntaxNode {
            position: SourceFilePosition { absolute: 20, line: 1, col: 21 },
            item: Box::new(AbstractSyntaxNodeItem::ProcedureBody(vec!(                       
                AbstractSyntaxNode {                    
                    position: SourceFilePosition { absolute: 26, line: 2, col: 5 },
                    item: Box::new(
                        AbstractSyntaxNodeItem::ProcedureCall {
                            name: "SomeOtherProcedure".to_string(),
                            args: vec!(),
                            arg_type: ResolvableType::Resolved(ResolvedTypeId::UserDefined(other_proc_type_id))
                        }
                    )
                }                        
            )))
        }
    );

}

