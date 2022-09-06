use crate::file_system::*;
use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;

pub struct ParserActor<T: FileRead> { file_reader: T }

pub fn create_parser_actor<T: FileRead>(file_reader: T) -> ParserActor<T>  {
    ParserActor { file_reader }
}

impl<T: FileRead> Actor<CompilationMessage> for ParserActor<T>  {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::ParseFile(file_name, compiler_handle) => handle_parse_file(&self.file_reader, file_name, compiler_handle),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_parse_file<T: FileRead>(file_reader: &T, file_name: String, compiler_handle: ActorHandle<CompilationMessage>) -> AfterReceiveAction {
    let result = parse_file(file_reader, &file_name);
    send_message_to_actor(&compiler_handle, create_file_parsed_event(result));
    shutdown_after_receive()
}

#[derive(Clone)]
pub enum FileParseResult {
    CompilationUnits { file_name: String, units: CompilationUnits },
    NotFound(String),
}

pub fn parse_file<T: FileRead>(file_reader: &T, file_name: &str) -> FileParseResult {
    match file_reader.read_file_to_string(file_name) {
        Ok(file_content) => FileParseResult::CompilationUnits { 
            file_name: file_name.to_string(), 
            units: parse(&file_content) 
        },
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
        SourceTokenItem::Identifier(name) => parse_top_level_identifier(name, lexer, token.position, units),
        SourceTokenItem::Directive(name) => parse_directive(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

pub fn create_node(item: AbstractSyntaxNodeItem, position: SourceFilePosition) -> AbstractSyntaxNode {
    AbstractSyntaxNode {
        item: Box::new(item),
        position,
    }
}

