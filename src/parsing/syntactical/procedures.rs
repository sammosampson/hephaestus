use crate::parsing::*;
use crate::typing::*;

pub fn parse_procedure_call(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let arguments = parse_procedure_call_args(lexer);
    
    assert!(is_close_paren(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    create_node(procedure_call_item(name, arguments, unresolved_resolvable_type(), vec!()), position)
}

fn parse_procedure_call_args(lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    let mut args = vec!();

    if is_close_paren(&peek_next_token(lexer).item) {
        return vec!()
    }

    loop {
        args.push(parse_procedure_call_arg(lexer));

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

fn parse_procedure_call_arg(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let expr = parse_procedure_call_arg_expr(lexer);
    let pos = expr.position;

    create_node(
        arg_item(expr, unresolved_resolvable_type()), 
        pos)        
}

pub fn parse_procedure_call_arg_expr(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

pub fn parse_procedure_header(name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    let args = parse_procedure_args(lexer);
    
    assert!(is_close_paren(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    let return_types = parse_procedure_return_types(lexer);
    let body = create_unit(parse_procedure_body(lexer, args.clone(), return_types.clone()));
    let body_ref = body.id;
    units.push(body);

    create_node(procedure_header_item(name, args, return_types, body_ref), position)
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
                return create_node(arg_declaration_item(name, arg_type), name_token.position)
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
        return create_node(type_item(return_type), next_token.position);
    }
    
    create_error_node(expected_type_error(), next_token.position)
}

fn parse_procedure_body(
    lexer: &mut Lexer,
    args: AbstractSyntaxChildNodes,
    return_types: AbstractSyntaxChildNodes
) -> AbstractSyntaxNode {
    if !is_open_brace(&peek_next_token(lexer).item) {
        return create_error_node(expected_open_brace_error(), get_next_token(lexer).position);
    }

    let brace = get_next_token(lexer);
    let statements = parse_procedure_body_statements(lexer);
    
    assert!(is_close_brace(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    create_node(procedure_body_item(args, return_types, statements), brace.position)
}

fn parse_procedure_body_statements(lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    if is_close_brace(&peek_next_token(lexer).item) {
        return vec!();
    }
    
    let mut statements = vec!();

    loop {
        statements.push(parse_procedure_body_statement(lexer));

        if is_line_terminiator(&peek_next_token(lexer).item) {
            eat_next_token(lexer)
        }

        if is_close_brace(&peek_next_token(lexer).item) {
            return statements
        }
    }
}

pub fn parse_procedure_body_statement(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

pub fn procedure_header_item(
    name: String,
    args: AbstractSyntaxChildNodes,
    return_types: AbstractSyntaxChildNodes,
    body_ref: CompilationUnitId
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureHeader { name, args, return_types, body: CompilationUnitReference::Resolved(body_ref) }
}

pub fn procedure_body_item(
    args: AbstractSyntaxChildNodes,
    return_types: AbstractSyntaxChildNodes,
    statements: AbstractSyntaxChildNodes
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureBody { 
        args,
        return_types,
        statements
    }
}

pub fn procedure_call_item(
    name: String,
    arguments: AbstractSyntaxChildNodes,
    type_id: ResolvableType,
    return_type_ids: ResolvableTypes
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureCall { name, args: arguments, type_id, return_type_ids }
}

pub fn arg_declaration_item(name: String, arg_type: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ArgumentDeclaration { name, type_id: arg_type }
}

pub fn arg_item(expr: AbstractSyntaxNode, arg_type: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Argument { expr, type_id: arg_type }
}

pub fn type_item(t: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Type(t)
}