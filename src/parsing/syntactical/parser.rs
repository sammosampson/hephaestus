use crate::file_system::*;
use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;
use crate::utilities::*;
use crate::errors::*;

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

#[derive(Clone, Debug)]
pub enum FileParseResult {
    CompilationUnits { file_name: String, units: CompilationUnits, errors: CompilationErrors },
    NotFound(String),
}

pub fn parse_file<T: FileRead>(file_reader: &T, filename: &str) -> FileParseResult {
    match file_reader.read_file_to_string(filename) {
        Ok(file_content) =>{
            let (units, errors) = parse(string(filename), &file_content);
            return FileParseResult::CompilationUnits { 
                file_name: filename.to_string(), 
                units,
                errors
            }
        },
        Err(_) => FileParseResult::NotFound(string(filename))
    }
}

pub fn parse(filename: String, input: &str) -> (CompilationUnits, CompilationErrors) {
    let mut lexer = lex(input);
    let mut units = vec!();
    let mut errors = create_compilation_errors(filename.clone());
        
    loop {
        match parse_next_node(filename.clone(), &mut lexer, &mut units) {
            Ok(node) => {
                if is_eof_node(&node) {
                    break;
                }      
                
                units.push(create_unit(filename.clone(), node));                  
            }
            Err(error) => {
                units.push(create_unit(filename.clone(), create_error_node(&error)));
                add_compilation_error(&mut errors, error);
                break;
            }
        }        
    }

    (units, errors)
}

fn is_eof_node(node: &AbstractSyntaxNode) -> bool {
    node.item_ref() == &AbstractSyntaxNodeItem::Eof
}

pub fn parse_next_node(filename: String, lexer: &mut Lexer, units: &mut CompilationUnits) -> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_top_level_identifier(filename, name, lexer, token.position, units),
        SourceTokenItem::Directive(name) => parse_directive(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => Err(create_error(tokenisation_error(error), token.position)),
        SourceTokenItem::Eof => Ok(create_node(create_eof_item(), token.position)),
        _ => Err(create_error(unimplemented_error(), token.position)),
    }
}

pub fn create_node(item: AbstractSyntaxNodeItem, position: SourceFilePosition) -> AbstractSyntaxNode {
    AbstractSyntaxNode {
        item: Box::new(item),
        position,
    }
}

