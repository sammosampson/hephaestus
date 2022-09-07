use crate::parsing::*;

pub fn parse_top_level_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    if is_declaration_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_top_level_declaration(name, lexer, position, units);
    }

    parse_remainder_of_identifier(name, lexer, position)
}


pub fn parse_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if is_declaration_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_declaration(name, lexer, position);
    }

    if is_initialise_assign_value_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_value_assignment(name, lexer, position);
    }

    parse_remainder_of_identifier(name, lexer, position)
}

pub fn parse_remainder_of_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if is_open_paren(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_procedure_call(name, lexer, position);
    }

    let node = create_node(create_identifier_item(name), position);
    
    if let SourceTokenItem::Operator(op) = peek_next_token(lexer).item {
        return parse_expression(lexer, op, node, position);
    }

    node
}

pub fn create_identifier_item(name: String) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Identifier { name }
}