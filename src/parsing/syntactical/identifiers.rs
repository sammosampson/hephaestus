use crate::parsing::*;

pub fn parse_top_level_identifier(filename: String, name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    if is_declaration_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_top_level_declaration(filename, name, lexer, position, units);
    }

    if is_initialise_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_known_type_top_level_declaration(name, lexer, position);
    }

    parse_remainder_of_identifier(name, lexer, position)
}


pub fn parse_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if is_declaration_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_inferred_constant(name, lexer, position);
    }

    if is_initialise_assign_value_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_inferred_value_assignment(name, lexer, position);
    }

    if is_initialise_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_initialise_assignment(name, lexer, position);
    }

    if is_period(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_struct_instance_access(name, lexer, position)
    }

    parse_remainder_of_identifier(name, lexer, position)
}

pub fn parse_remainder_of_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if is_open_paren(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_procedure_call(name, lexer, position);
    }

    let node = create_node(unknown_scope_identifier_item(name), position);
    
    if is_operator(&peek_next_token(lexer).item) {
        return parse_expression(lexer, node, position);
    }

    node
}

pub fn unknown_scope_identifier_item(name: String) -> AbstractSyntaxNodeItem {
    identifier_item(name, unknown_scope())
}

pub fn identifier_item(name: String, scope: Scope) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Identifier { name, scope }
}