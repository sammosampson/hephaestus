use crate::parsing::*;
use crate::compilation::*;

pub fn parse_top_level_identifier(
    filename: String,
    name: String,
    lexer: &mut Lexer,
    position: SourceFilePosition,
    units: &mut CompilationUnits,
    errors: &mut CompilationErrors
) -> AbstractSyntaxNode {
    if is_declaration_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_top_level_declaration(filename, name, lexer, position, units, errors);
    }

    if is_initialise_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_known_type_top_level_declaration(name, lexer, position, errors);
    }

    parse_remainder_of_identifier(name, lexer, position, errors)
}


pub fn parse_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    if is_declaration_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_inferred_constant(name, lexer, position, errors);
    }

    if is_initialise_assign_value_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_inferred_value_assignment(name, lexer, position, errors);
    }

    if is_initialise_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_initialise_assignment(name, lexer, position, errors);
    }

    if is_period(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_struct_instance_access(name, lexer, position, errors)
    }

    parse_remainder_of_identifier(name, lexer, position, errors)
}

pub fn parse_remainder_of_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    if is_open_paren(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_procedure_call(name, lexer, position, errors);
    }

    let node = create_node(unknown_scope_identifier_item(name), position);
    
    if is_operator(&peek_next_token(lexer).item) {
        return parse_expression(lexer, node, position, errors);
    }

    node
}

pub fn unknown_scope_identifier_item(name: String) -> AbstractSyntaxNodeItem {
    identifier_item(name, unknown_scope())
}

pub fn identifier_item(name: String, scope: Scope) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Identifier { name, scope }
}