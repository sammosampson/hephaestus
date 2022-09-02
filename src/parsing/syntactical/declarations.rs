use crate::parsing::*;

pub fn parse_declaration(name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    if !is_open_paren(&peek_next_token(lexer).item) {
        return create_node(create_constant_item(name, parse_next_node(lexer, units)), position); 
    } 
    eat_next_token(lexer);

    parse_procedure_header(name, lexer, position, units)
}

fn create_constant_item(name: String, value: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Constant { name, value }
}