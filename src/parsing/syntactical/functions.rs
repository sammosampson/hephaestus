use crate::parsing::*;

pub fn parse_procedure_call(name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    let arguments = parse_procedure_call_args(lexer, units);
    
    assert!(is_close_paren(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    create_node(create_procedure_call_item(name, arguments), position)
}

fn parse_procedure_call_args(lexer: &mut Lexer, units: &mut CompilationUnits) -> Vec<AbstractSyntaxNode> {
    let mut args = vec!();

    if is_close_paren(&peek_next_token(lexer).item) {
        return vec!()
    }

    loop {
        args.push(parse_procedure_call_arg(lexer, units));

        let next_token = peek_next_token(lexer);
        
        if is_close_paren(&next_token.item) {
            return args
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            args.push(create_error_node(expected_arg_separator_error(), next_token.position));  
            return args;
        }
    }
}

fn parse_procedure_call_arg(lexer: &mut Lexer, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    let expr = parse_next_node(lexer, units);
    let pos = expr.position;

    create_node(
        create_arg_item(
            expr, 
            Type::Unknown
        ), 
        pos)        
}

pub fn parse_procedure_header(name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    let args = parse_procedure_args(lexer);
    
    assert!(is_close_paren(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    let return_types = parse_procedure_return_types(lexer);
    let body = create_unit(parse_procedure_body(lexer));
    let body_ref = body.id;
    units.push(body);

    create_node(create_procedure_header_item(name, args, return_types, body_ref), position)
}

fn parse_procedure_args(lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    let mut args = vec!();

    if is_close_paren(&peek_next_token(lexer).item) {
        return vec!()
    }

    loop {
        args.push(parse_procedure_arg(lexer));

        let next_token = peek_next_token(lexer);
        
        if is_close_paren(&next_token.item) {
            return args
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            args.push(create_error_node(expected_arg_separator_error(), next_token.position));  
            return args;
        }
    }
}

fn parse_procedure_arg(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let name_token = peek_next_token(lexer);
    if let Some(name) = try_get_identifier(name_token.item) {
        eat_next_token(lexer);
        
        if is_initialise_assignment(&peek_next_token(lexer).item) {
            eat_next_token(lexer);
        
            if let Some(arg_type) = try_get_type(&peek_next_token(lexer).item) {
                eat_next_token(lexer);
                return create_node(create_arg_declaraton_item(name, arg_type), name_token.position)
            }

            return create_error_node(unimplemented_error(), peek_next_token(lexer).position);        
        }

        return create_error_node(expected_initialise_assignment_error(), peek_next_token(lexer).position);        
    }
    
    create_error_node(expected_arg_name_error(), peek_next_token(lexer).position)
}

fn parse_procedure_return_types(lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    if !is_goes_to_assignment(&peek_next_token(lexer).item) {
        return vec!();
    }
    eat_next_token(lexer);

    let mut returns = vec!();

    if is_open_brace(&peek_next_token(lexer).item) {
        return vec!()
    }

    loop {
        returns.push(parse_procedure_return_type(lexer));

        let next_token = peek_next_token(lexer);
        
        if is_open_brace(&next_token.item) {
            return returns
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            returns.push(create_error_node(expected_arg_separator_error(), next_token.position));  
            return returns;
        }
    }
}

fn parse_procedure_return_type(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let next_token = get_next_token(lexer);

    if let Some(return_type) = try_get_type(&next_token.item) {
        return create_node(create_type_item(return_type), next_token.position);
    }
    
    create_error_node(expected_type_error(), next_token.position)
}

fn parse_procedure_body(lexer: &mut Lexer) -> AbstractSyntaxNode {
    if !is_open_brace(&peek_next_token(lexer).item) {
        return create_error_node(expected_open_brace_error(), get_next_token(lexer).position);
    }

    let brace = get_next_token(lexer);
    let children = parse_procedure_body_nodes(lexer);
    
    assert!(is_close_brace(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    create_node(create_procedure_body_item(children), brace.position)
}

fn parse_procedure_body_nodes(_lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    vec!()
}

fn create_procedure_header_item(
    name: String,
    arguments: AbstractSyntaxChildNodes,
    return_types: AbstractSyntaxChildNodes,
    body_ref: CompilationUnitId
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureHeader { name, arguments, return_types, body: CompilationUnitReference::Resolved(body_ref) }
}

fn create_procedure_body_item(children: AbstractSyntaxChildNodes) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureBody(children)
}

fn create_procedure_call_item(name: String, arguments: AbstractSyntaxChildNodes) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureCall { name, arguments, arg_type: Type::Unknown }
}

fn create_arg_declaraton_item(name: String, arg_type: Type) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ArgumentDeclaration { name, arg_type }
}

fn create_arg_item(expr: AbstractSyntaxNode, arg_type: Type) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Argument { expr, arg_type }
}

fn create_type_item(t: Type) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Type(t)
}