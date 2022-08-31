use crate::parsing::*;

pub fn parse_declaration(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if !is_open_paren(&peek_next_token(lexer).item) {
        return create_node(create_constant_item(name, parse_next_node(lexer)), position); 
    } 
    eat_next_token(lexer);

    parse_function_header(name, lexer, position)
}

fn create_constant_item(name: String, value: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Constant { name, value }
}