use crate::parsing::*;

pub fn parse_directive(directive: Directive, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    match directive {
        Directive::Run => parse_run_directive(lexer, position),
        Directive::Load => parse_load_directive(lexer, position)
    }
}

fn parse_run_directive(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(create_run_directive_item(parse_run_directive_expr(lexer)), position)
}


pub fn parse_run_directive_expr(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

fn parse_load_directive(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);
    if let Some(file_name) = try_get_string_literal(&token.item) {
        return create_node(create_load_directive_item(file_name), position);
    }
    create_error_node(expected_file_name_error(), token.position)
}


fn create_run_directive_item(expr: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Run { expr }
}

fn create_load_directive_item(file_name: String) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Load { file_name }
}