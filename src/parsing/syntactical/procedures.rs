use crate::parsing::*;
use crate::types::*;

#[derive(PartialEq, Debug, Clone)]
pub enum ProcedureBodyReference {
    Unknown,
    Local(CompilationUnitId),
    Foerign(AbstractSyntaxNode)
}

pub fn unknown_procedure_body_reference() -> ProcedureBodyReference {
    ProcedureBodyReference::Unknown
}

pub fn local_procedure_body_reference(id: CompilationUnitId) -> ProcedureBodyReference {
    ProcedureBodyReference::Local(id)
}

pub fn foreign_procedure_body_reference(foreign_system_library: AbstractSyntaxNode) -> ProcedureBodyReference {
    ProcedureBodyReference::Foerign(foreign_system_library)
}

pub fn parse_procedure_call(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    let arguments = parse_procedure_call_args(lexer)?;
    
    assert!(is_close_paren(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    Ok(create_node(procedure_call_item(name, arguments, unresolved_resolvable_type()), position))
}

fn parse_procedure_call_args(lexer: &mut Lexer) -> AbstractSyntaxChildNodesResult {
    let mut args = vec!();

    if is_close_paren(&peek_next_token(lexer).item) {
        return Ok(vec!());
    }

    loop {
        args.push(parse_procedure_call_arg(lexer)?);

        let next_token = peek_next_token(lexer);
        
        if is_close_paren(&next_token.item) {
            return Ok(args);
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            return Err(create_error(expected_arg_separator_error(), next_token.position));  
        }
    }
}

fn parse_procedure_call_arg(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    let expr = parse_procedure_call_arg_expr(lexer)?;
    let pos = expr.position;

    Ok(create_node(arg_item(expr, unresolved_resolvable_type()), pos))
}

pub fn parse_procedure_call_arg_expr(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Keyword(keyword) => parse_procedure_call_keyword(keyword, token.position),
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => Err(create_error(tokenisation_error(error), token.position)),
        SourceTokenItem::Eof => Ok(create_node(create_eof_item(), token.position)),
        _ => Err(create_error(unimplemented_error(), token.position)),
    }
}

fn parse_procedure_call_keyword(keyword: Keyword, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    match keyword {
        Keyword::Null => Ok(create_node(null_item(), position)),
        _ => Err(create_error(unimplemented_error(), position)),
    }
}

pub fn parse_procedure_header(filename: String, name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNodeResult {
    let args = parse_procedure_args(lexer)?;
    
    assert!(is_close_paren(&peek_next_token(lexer).item));
    
    eat_next_token(lexer);

    let return_types = parse_procedure_return_types(lexer)?;

    let mut body_ref = unknown_procedure_body_reference();

    if is_open_brace(&peek_next_token(lexer).item) {
        let body = create_unit(
            filename, 
            parse_procedure_body(lexer, name.clone(), args.clone(), return_types.clone())?
        );
        body_ref = local_procedure_body_reference(body.id);
        units.push(body);
    }

    if is_foreign_directive(&peek_next_token(lexer).item) {
        let foreign_library_identifier = parse_foreign_library_identifier(lexer)?;
        body_ref = foreign_procedure_body_reference(foreign_library_identifier);
        
        if is_line_terminiator(&peek_next_token(lexer).item) {
            eat_next_token(lexer);
        }
    }

    Ok(create_node(procedure_header_item(name, args, return_types, body_ref), position))
}

fn parse_procedure_args(lexer: &mut Lexer) -> AbstractSyntaxChildNodesResult {
    let mut args = vec!();

    if is_close_paren(&peek_next_token(lexer).item) {
        return Ok(vec!())
    }

    loop {
        args.push(parse_declaration(lexer)?);

        let next_token = peek_next_token(lexer);
        
        if is_close_paren(&next_token.item) {
            return Ok(args)
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            return Err(create_error(expected_arg_separator_error(), next_token.position));  
        }
    }
}

fn parse_procedure_return_types(lexer: &mut Lexer) -> AbstractSyntaxChildNodesResult {
    if !is_goes_to_assignment(&peek_next_token(lexer).item) {
        return Ok(vec!());
    }
    eat_next_token(lexer);

    let mut returns = vec!();

    if is_open_brace(&peek_next_token(lexer).item) {
        return Ok(vec!())
    }

    loop {
        returns.push(parse_procedure_return_type(lexer)?);

        let next_token = peek_next_token(lexer);
        
        if is_open_brace(&next_token.item) {
            return Ok(returns)
        }

        if is_foreign_directive(&next_token.item) {
            return Ok(returns)
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            return Err(create_error(expected_arg_separator_error(), next_token.position));  
        }
    }
}

fn parse_procedure_return_type(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    if let Some(return_type) = try_parse_type(lexer) {
        let next_token = get_next_token(lexer);
        return Ok(create_node(type_item(return_type), next_token.position));
    }

    Err(create_error(expected_type_error(), get_next_token(lexer).position))
}

fn parse_procedure_body(
    lexer: &mut Lexer,
    name: String,
    args: AbstractSyntaxChildNodes,
    return_types: AbstractSyntaxChildNodes
) -> AbstractSyntaxNodeResult {
    assert!(is_open_brace(&peek_next_token(lexer).item));

    let brace = get_next_token(lexer);
    let statements = parse_procedure_body_statements(lexer)?;
    
    assert!(is_close_brace(&peek_next_token(lexer).item));

    eat_next_token(lexer);

    Ok(create_node(procedure_body_item(name, args, return_types, statements), brace.position))
}

fn parse_foreign_library_identifier(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    assert!(is_foreign_directive(&peek_next_token(lexer).item));
    eat_next_token(lexer);
        
    let token = peek_next_token(lexer);
        
    if let Some(foreign_library) = try_get_identifier(token.item) {
        eat_next_token(lexer);
        return Ok(create_node(unknown_scope_identifier_item(foreign_library), token.position));
    }

    Err(create_error(expected_foreign_library_identifier_error(), token.position))
}

fn parse_procedure_body_statements(lexer: &mut Lexer) -> AbstractSyntaxChildNodesResult {
    if is_close_brace(&peek_next_token(lexer).item) {
        return Ok(vec!());
    }
    
    let mut statements = vec!();

    loop {
        statements.push(parse_procedure_body_statement(lexer)?);

        if is_line_terminiator(&peek_next_token(lexer).item) {
            eat_next_token(lexer)
        }

        if is_close_brace(&peek_next_token(lexer).item) {
            return Ok(statements);
        }
    }
}

pub fn parse_procedure_body_statement(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Keyword(keyword) => parse_procedure_body_keyword(keyword, lexer, token.position),
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Error(error) => Err(create_error(tokenisation_error(error), token.position)),
        SourceTokenItem::Eof => Ok(create_node(create_eof_item(), token.position)),
        _ => Err(create_error(unimplemented_error(), token.position)),
    }
}

fn parse_procedure_body_keyword(keyword: Keyword, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    match keyword {
        Keyword::Return => parse_return_statement(lexer, position),
        _ => Err(create_error(unimplemented_error(), position)),
    }
}

fn parse_return_statement(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    Ok(create_node(
        return_item(parse_return_statement_args(lexer)?),
        position
    ))
}

fn parse_return_statement_args(lexer: &mut Lexer) -> AbstractSyntaxChildNodesResult {
    let mut args = vec!();

    if is_line_terminiator(&peek_next_token(lexer).item) {
        return Ok(vec!())
    }

    loop {
        args.push(parse_procedure_call_arg(lexer)?);

        let next_token = peek_next_token(lexer);
        
        if is_line_terminiator(&next_token.item) {
            return Ok(args);
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            return Err(create_error(expected_arg_separator_error(), next_token.position));  
        }
    }
}

pub fn procedure_header_item(
    name: String,
    args: AbstractSyntaxChildNodes,
    return_types: AbstractSyntaxChildNodes,
    body: ProcedureBodyReference
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureHeader { name, args, return_args: return_types, body }
}

pub fn procedure_body_item(
    name: String,
    args: AbstractSyntaxChildNodes,
    return_types: AbstractSyntaxChildNodes,
    statements: AbstractSyntaxChildNodes
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureBody { 
        name,
        args,
        return_types,
        statements
    }
}

pub fn return_item(
    args: AbstractSyntaxChildNodes
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Return { args }
}

pub fn null_item() -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Null
}

pub fn procedure_call_item(
    name: String,
    args: AbstractSyntaxChildNodes,
    type_id: ResolvableType
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ProcedureCall { name, args, procedure_call_type: type_id }
}

pub fn arg_item(expr: AbstractSyntaxNode, type_id: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Argument { expr, arg_type: type_id }
}

pub fn type_item(t: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Type(t)
}