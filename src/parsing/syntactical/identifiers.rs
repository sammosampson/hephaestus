use crate::parsing::*;

pub fn parse_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    if is_declaration_assignment(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_declaration(name, lexer, position, units);
    }

    if is_open_paren(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_procedure_call(name, lexer, position, units);
    }

    create_node(create_identifier_item(name), position)
}

pub fn create_identifier_item(name: String) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Identifier(name)
}