
use crate::typing::*;
use crate::parsing::*;

#[test]
fn typing_procedure_body_waits_for_external_procedure() {
    let mut units = crate::tests::parsing::run_parse_file_return_only_units("SomeProcedure :: () {
    SomeExternalProcedure(1);
}");

    let external_proc_type = crate::tests::typing::create_procedure_definition_type(
        "SomeExternalProcedure",
        vec!(
            ResolvedTypeId::BuiltInType(BuiltInType::Int32)
        ),
        vec!()
    );
    
    let external_proc_type_id = external_proc_type.id.clone();

    let typing_repository = crate::tests::typing::start_type_repository_actor();
    
    
    crate::tests::typing::add_resolved_type(
        &typing_repository, 
        crate::tests::typing::create_procedure_definition_type_with_no_args("SomeExternalProcedure")
    );

    crate::tests::typing::add_resolved_type(&typing_repository, external_proc_type);
    
    crate::tests::typing::add_resolved_type(
        &typing_repository, 
        crate::tests::typing::create_procedure_definition_type_with_no_args("SomeOtherExternalProcedure")
    );

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
                            name: "SomeExternalProcedure".to_string(),
                            args: vec!(
                                AbstractSyntaxNode {
                                    position: SourceFilePosition { absolute: 48, line: 2, col: 27 },
                                    item: Box::new(
                                        AbstractSyntaxNodeItem::Argument { 
                                            expr: AbstractSyntaxNode {
                                                position: SourceFilePosition { absolute: 48, line: 2, col: 27 },
                                                item: Box::new(
                                                    AbstractSyntaxNodeItem::Literal(Literal::Int(1))
                                                )
                                            },
                                            type_id: ResolvableType::Resolved(ResolvedTypeId::BuiltInType(BuiltInType::Int32)) 
                                        }
                                    ),
                                }
                            ),
                            type_id: ResolvableType::Resolved(external_proc_type_id)
                        }
                    )
                }                        
            )))
        }
    );

}
