use crate::parsing::*;

pub fn parse_top_level_declaration(filename: String, name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    if is_open_paren(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_procedure_header(filename, name, lexer, position, units)
    } 

    parse_declaration(name, lexer, position)
}

pub fn parse_declaration(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(constant_item(name, parse_constant_value(lexer)), position)
}

pub fn parse_constant_value(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Directive(directive) => parse_const_directive(directive, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

pub fn constant_item(name: String, value: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Constant { name, value }
}