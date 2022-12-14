use crate::parsing::*;
use crate::types::*;

pub fn parse_inferred_constant(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    parse_constant(name, lexer, position, unresolved_resolvable_type())
}

pub fn parse_constant(name: String, lexer: &mut Lexer, position: SourceFilePosition, resolvable_type: ResolvableType) -> AbstractSyntaxNodeResult {
    let node = create_node(
        constant_item(name, 
            parse_constant_value(lexer)?, 
            resolvable_type
        ),
        position
    );
    
    if is_line_terminiator(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
    }
    
    Ok(node)
}

fn parse_constant_value(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Directive(directive) => parse_const_directive(directive, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => Err(create_error(tokenisation_error(error), token.position)),
        SourceTokenItem::Eof => Ok(create_node(create_eof_item(), token.position)),
        _ => Err(create_error(unimplemented_error(), token.position)),
    }
}

pub fn constant_item(name: String, value: AbstractSyntaxNode, constant_type: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Constant { name, value, constant_type }
}
