use crate::file_system::*;
use crate::parsing::*;

#[derive(Clone)]
pub enum FileParseResult {
    CompilationUnits { file_name: String, units: CompilationUnits },
    NotFound(String),
}

pub fn parse_file(file_name: &str) -> FileParseResult {
    match read_file_to_string(file_name) {
        Ok(file_content) => FileParseResult::CompilationUnits { file_name: file_name.to_string(), units: parse(&file_content) },
        Err(_) => FileParseResult::NotFound(file_name.to_string())
    }
}

pub fn parse(input: &str) -> CompilationUnits {
    let mut lexer = lex(input);
    let mut units = vec!();

    loop {
        let node = parse_next_node(&mut lexer, &mut units);
        
        if is_eof_node(&node) {
            break;
        }      
        
        units.push(create_unit(node))
    }

    units
}

fn is_eof_node(node: &AbstractSyntaxNode) -> bool {
    node.item_ref() == &AbstractSyntaxNodeItem::Eof
}

pub fn parse_next_node(lexer: &mut Lexer, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position, units),
        SourceTokenItem::Directive(name) => parse_directive(name, lexer, token.position, units),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position, units),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

fn create_eof_item() -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Eof
}

pub fn create_node(item: AbstractSyntaxNodeItem, position: SourceFilePosition) -> AbstractSyntaxNode {
    AbstractSyntaxNode {
        item: Box::new(item),
        position,
    }
}

