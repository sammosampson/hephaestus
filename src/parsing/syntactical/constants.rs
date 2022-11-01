use crate::parsing::*;
use crate::compilation::*;
use crate::types::*;

pub fn parse_inferred_constant(name: String, lexer: &mut Lexer, position: SourceFilePosition, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    parse_constant(name, lexer, position, unresolved_resolvable_type(), errors)
}

pub fn parse_constant(name: String, lexer: &mut Lexer, position: SourceFilePosition, resolvable_type: ResolvableType, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    let node = create_node(
        constant_item(name, 
            parse_constant_value(lexer, errors), 
            resolvable_type
        ),
        position
    );
    
    if is_line_terminiator(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
    }
    
    node
}

fn parse_constant_value(lexer: &mut Lexer, errors: &mut CompilationErrors) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Directive(directive) => parse_const_directive(directive, lexer, token.position, errors),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position, errors),
        SourceTokenItem::Error(error) => create_error_and_error_node(errors, tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_and_error_node(errors, unimplemented_error(), token.position),
    }
}

pub fn constant_item(name: String, value: AbstractSyntaxNode, constant_type: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Constant { name, value, constant_type }
}
