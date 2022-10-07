use crate::parsing::*;
use crate::typing::*;

pub fn parse_top_level_declaration(filename: String, name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    if is_open_paren(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_procedure_header(filename, name, lexer, position, units)
    } 

    parse_inferred_constant(name, lexer, position)
}

pub fn parse_known_type_top_level_declaration(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if let Some(resolvable_type) = try_parse_type(lexer) {
        eat_next_token(lexer);
        if is_initialise_assignment( &peek_next_token(lexer).item) {
            eat_next_token(lexer);
            return parse_constant(name, lexer, position, resolvable_type);
        }        
        return create_error_node(expected_initialise_assignment_error(), get_next_token(lexer).position);
    }
    create_error_node(expected_type_error(), get_next_token(lexer).position)
}

pub fn parse_inferred_constant(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    parse_constant(name, lexer, position, unresolved_resolvable_type())
}

fn parse_constant(name: String, lexer: &mut Lexer, position: SourceFilePosition, resolvable_type: ResolvableType) -> AbstractSyntaxNode {
    let node = create_node(
        constant_item(name, 
            parse_constant_value(lexer), 
            resolvable_type
        ),
        position
    );
    
    if is_line_terminiator(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
    }
    
    node
}

fn parse_constant_value(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Directive(directive) => parse_const_directive(directive, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

pub fn constant_item(name: String, value: AbstractSyntaxNode, constant_type: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Constant { name, value, constant_type }
}