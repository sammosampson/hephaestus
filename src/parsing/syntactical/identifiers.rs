use crate::parsing::*;

pub fn parse_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if !is_declaration_assignment(&peek_next_token(lexer).item) {
        return create_error_node(unimplemented_error(), position);
    }
    eat_next_token(lexer);

    parse_declaration(name, lexer, position)
}