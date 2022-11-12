use crate::file_system::*;
use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;
use crate::utilities::*;
use crate::errors::*;

pub struct ParserActor<T: FileRead> {
    compiler: CompilationActorHandle,
    error_reporter: CompilationActorHandle,
    file_reader: T }

pub fn create_parser_actor<T: FileRead>(compiler: CompilationActorHandle, error_reporter: CompilationActorHandle, file_reader: T) -> ParserActor<T>  {
    ParserActor { 
        compiler, 
        error_reporter,
        file_reader
    }
}

impl<T: FileRead> Actor<CompilationMessage> for ParserActor<T>  {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::ParseFile(file_name) => handle_parse_file(&self.compiler, &self.error_reporter, &self.file_reader, file_name),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_parse_file<T: FileRead>(compiler: &CompilationActorHandle, error_reporter: &CompilationActorHandle, file_reader: &T, file_name: String) -> AfterReceiveAction {
    let units = parse_file(compiler, error_reporter, file_reader, &file_name);
    send_message_to_actor(compiler, create_file_parsed_event(file_name, units));
    shutdown_after_receive()
}

fn parse_file<T: FileRead>(compiler: &CompilationActorHandle, error_reporter: &CompilationActorHandle, file_reader: &T, filename: &str) -> CompilationUnits {
    match file_reader.read_file_to_string(filename) {
        Ok(file_content) =>{
            let (units, errors) = parse(string(filename), &file_content);
            report_errors(error_reporter, compiler.clone(), errors);
            return units;
        },
        Err(_) => {
            report_errors(error_reporter, compiler.clone(), create_errors_for_file_not_found(string(filename)));
            return create_compilation_units();
        }
    }
}

fn create_errors_for_file_not_found(filename: String) -> CompilationErrors {
    let mut errors = create_compilation_errors(filename.clone());
    add_compilation_error(&mut errors, compilation_error(file_not_found_error(filename), no_position()));
    errors
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

