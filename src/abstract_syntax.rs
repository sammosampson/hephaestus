use crate::{
    tokenisation::*,
    source_files::*,
    literals::*,
    operators::*, 
    directives::*,
    file_system::*,
    enclosures::*,
    terminators::*,
    types::*,
};

#[test]
fn parse_empty_input_parses_correctly() {
    let ast = parse("");
    assert_eq!(1, ast.children.len());
    let child = &ast.children[0];
    assert_eq!(child.item_ref().clone(), AbstractSyntaxNodeItem::Eof);
}

#[test]
fn parse_run_expression_parses_correctly() {
    let ast = parse("#run 1 + 2");
    match ast.children[0].item_ref() {
        AbstractSyntaxNodeItem::Run { expr } => match expr.item_ref() {
            AbstractSyntaxNodeItem::BinaryExpr { op, lhs, rhs } => {
                assert_eq!(&Operator::Add, op);
                assert_eq!(&AbstractSyntaxNodeItem::Literal(Literal::Int(1)), lhs.item_ref());
                assert_eq!(&AbstractSyntaxNodeItem::Literal(Literal::Int(2)), rhs.item_ref());
            },
            _ => crate::testing::assert_fail("BinaryExpr not returned"),
        },
        _ => crate::testing::assert_fail("Run not returned"),
    }
}

#[test]
fn parse_load_expression_parses_correctly() {
    let ast = parse("#load \"test.jai\"");
    
    match ast.children[0].item_ref() {
        AbstractSyntaxNodeItem::Load { file_name } => assert_eq!(file_name, "test.jai"),
        _ => crate::testing::assert_fail("Load not returned"),
    }
}

#[test]
fn parse_const_declaration_parses_correctly() {
    let ast = parse("SomeValue :: 1");

    match ast.children[0].item_ref() {
        AbstractSyntaxNodeItem::Constant { name, value } => {
            assert_eq!(name, "SomeValue");
            assert_eq!(&AbstractSyntaxNodeItem::Literal(Literal::Int(1)), value.item_ref());
        },
        _ => crate::testing::assert_fail("Load not returned"),
    }    
}

#[test]
fn parse_function_declaration_parses_correctly() {
    let ast = parse("SomeFunction :: () {
}");
    
    assert_eq!(
        ast.children[0], 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::FunctionHeader {
                    name: "SomeFunction".to_string(),
                    arguments: vec!(),
                    return_types: vec!(), 
                    body: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                        position: SourceFilePosition { absolute: 19, line: 1, col: 20 }
                    }
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_function_declaration_with_return_type_parses_correctly() {
    let ast = parse("SomeFunction :: () -> void {
}");
    
    assert_eq!(
        ast.children[0], 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::FunctionHeader {
                    name: "SomeFunction".to_string(),
                    arguments: vec!(),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Void))),
                            position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                        }
                    ),
                    body: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                        position: SourceFilePosition { absolute: 27, line: 1, col: 28 }
                    }
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_function_declaration_with_return_types_parses_correctly() {
    let ast = parse("SomeFunction :: () -> SomeType, int {
}");
    
    assert_eq!(
        ast.children[0], 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::FunctionHeader {
                    name: "SomeFunction".to_string(),
                    arguments: vec!(),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::Compound("SomeType".to_string()))),
                            position: SourceFilePosition { absolute: 22, line: 1, col: 23 }
                        },
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Int))),
                            position: SourceFilePosition { absolute: 32, line: 1, col: 33 }
                        }
                    ),
                    body: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                        position: SourceFilePosition { absolute: 36, line: 1, col: 37 }
                    }
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

#[test]
fn parse_function_declaration_with_arg_parses_correctly() {
    let ast = parse("SomeFunction :: (x: int) {
}");
    
assert_eq!(
    ast.children[0], 
    AbstractSyntaxNode {
        item: Box::new(
            AbstractSyntaxNodeItem::FunctionHeader {
                name: "SomeFunction".to_string(),
                arguments: vec!(
                    AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "x".to_string() , arg_type: Type::BuiltIn(BuiltInType::Int) }),
                        position: SourceFilePosition { absolute: 17, line: 1, col: 18 }
                    }
                ),
                return_types: vec!(), 
                body: AbstractSyntaxNode {
                    item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                    position: SourceFilePosition { absolute: 25, line: 1, col: 26 }
                }
            }
        ),
        position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
    }
);
}

#[test]
fn parse_function_declaration_with_args_and_return_type_parses_correctly() {
    let ast = parse("SomeFunction :: (x: float, y: SomeType) -> void {
}");
    
    assert_eq!(
        ast.children[0], 
        AbstractSyntaxNode {
            item: Box::new(
                AbstractSyntaxNodeItem::FunctionHeader {
                    name: "SomeFunction".to_string(),
                    arguments: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "x".to_string() , arg_type: Type::BuiltIn(BuiltInType::Float) }),
                            position: SourceFilePosition { absolute: 17, line: 1, col: 18 }
                        },
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::ArgumentDeclaration { name: "y".to_string() , arg_type: Type::Compound("SomeType".to_string()) }),
                            position: SourceFilePosition { absolute: 27, line: 1, col: 28 }
                        }
                    ),
                    return_types: vec!(
                        AbstractSyntaxNode {
                            item: Box::new(AbstractSyntaxNodeItem::Type(Type::BuiltIn(BuiltInType::Void))),
                            position: SourceFilePosition { absolute: 43, line: 1, col: 44 }
                        }
                    ), 
                    body: AbstractSyntaxNode {
                        item: Box::new(AbstractSyntaxNodeItem::FunctionBody(vec!())),
                        position: SourceFilePosition { absolute: 48, line: 1, col: 49 }
                    }
                }
            ),
            position: SourceFilePosition { absolute: 0, line: 1, col: 1 }
        }
    );
}

pub type AbstractSyntaxChildNodeItem = Box<AbstractSyntaxNodeItem>;
pub type AbstractSyntaxChildNodes = Vec<AbstractSyntaxNode>;

#[derive(PartialEq, Debug)]
pub struct AbstractSyntaxTree {
    children: AbstractSyntaxChildNodes,
}

fn create_tree(children: Vec<AbstractSyntaxNode>) -> AbstractSyntaxTree {
    AbstractSyntaxTree { children }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AbstractSyntaxNode {
    pub position: SourceFilePosition,
    item: AbstractSyntaxChildNodeItem,
}

impl AbstractSyntaxNode {
    fn item_ref(&self) -> &AbstractSyntaxNodeItem {
        self.item.as_ref()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AbstractSyntaxNodeItem {
    Run { expr: AbstractSyntaxNode },
    Load { file_name: String },
    FunctionHeader {
        name: String,
        arguments: AbstractSyntaxChildNodes,
        return_types: AbstractSyntaxChildNodes,
        body: AbstractSyntaxNode
    },
    FunctionBody(AbstractSyntaxChildNodes),
    ArgumentDeclaration { name: String, arg_type: Type },
    Type(Type),
    Constant {
        name: String,
        value: AbstractSyntaxNode
    },
    Literal(Literal),
    BinaryExpr {
        op: Operator,
        lhs: AbstractSyntaxNode,
        rhs: AbstractSyntaxNode,
    },
    Error(AbstractSyntaxParseError),
    Eof
}


pub fn parse_file(file_name: String) -> AbstractSyntaxTree {
    match read_file_to_string(&file_name) {
        Ok(file_content) => parse(&file_content),
        Err(_) => create_tree(vec!(create_error_node(file_not_found_error(file_name), empty_position())))
    }
}

fn parse(input: &str) -> AbstractSyntaxTree {
    let mut lexer = lex(input);
    let mut children = vec!();

    loop {
        let node = parse_next_node(&mut lexer);
        
        if is_eof_node(&node) {
            children.push(node);
            break;
        }
        
        children.push(node);
         
    }

    create_tree(children)
}

fn is_eof_node(node: &AbstractSyntaxNode) -> bool {
    node.item_ref() == &AbstractSyntaxNodeItem::Eof
}

fn parse_next_node(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) => parse_identifier(name, lexer, token.position),
        SourceTokenItem::Directive(name) => parse_directive(name, lexer, token.position),
        SourceTokenItem::Literal(literal) => parse_literal(literal, lexer, token.position),
        SourceTokenItem::Error(error) => create_error_node(tokenisation_error(error), token.position),
        SourceTokenItem::Eof => create_node(create_eof_item(), token.position),
        _ => create_error_node(unimplemented_error(), token.position),
    }
}

fn parse_directive(directive: Directive, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    match directive {
        Directive::Run => parse_run_directive(lexer, position),
        Directive::Load => parse_load_directive(lexer, position)
    }
}

fn parse_identifier(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if !is_declaration_assignment(&peek_next_token(lexer).item) {
        return create_error_node(unimplemented_error(), position);
    }
    eat_next_token(lexer);

    parse_declaration(name, lexer, position)
}

fn parse_declaration(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if !is_open_paren(&peek_next_token(lexer).item) {
        return create_node(create_constant_item(name, parse_next_node(lexer)), position); 
    } 
    eat_next_token(lexer);

    parse_function_header(name, lexer, position)
}

fn parse_function_header(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let args = parse_function_args(lexer);
    
    assert!(is_close_paren(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    let return_types = parse_function_return_types(lexer);
    let body = parse_function_body(lexer);

    create_node(create_function_declaration_item(name, args, return_types, body), position)
}

fn parse_function_args(lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    let mut args = vec!();

    if is_close_paren(&peek_next_token(lexer).item) {
        return vec!()
    }

    loop {
        args.push(parse_function_arg(lexer));

        let next_token = peek_next_token(lexer);
        
        if is_close_paren(&next_token.item) {
            return args
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            args.push(create_error_node(expected_arg_separator_error(), next_token.position));  
            return args;
        }
    }
}


fn parse_function_arg(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let name_token = peek_next_token(lexer);
    if let Some(name) = try_get_identifier(name_token.item) {
        eat_next_token(lexer);
        
        if is_initialise_assignment(&peek_next_token(lexer).item) {
            eat_next_token(lexer);
        
            if let Some(arg_type) = try_get_type(&peek_next_token(lexer).item) {
                eat_next_token(lexer);
                return create_node(create_arg_item(name, arg_type), name_token.position)
            }

            return create_error_node(unimplemented_error(), peek_next_token(lexer).position);        
        }

        return create_error_node(expected_initialise_assignment_error(), peek_next_token(lexer).position);        
    }
    
    create_error_node(expected_arg_name_error(), peek_next_token(lexer).position)
}

fn parse_function_return_types(lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    if !is_goes_to_assignment(&peek_next_token(lexer).item) {
        return vec!();
    }
    eat_next_token(lexer);

    let mut returns = vec!();

    if is_open_brace(&peek_next_token(lexer).item) {
        return vec!()
    }

    loop {
        returns.push(parse_function_return_type(lexer));

        let next_token = peek_next_token(lexer);
        
        if is_open_brace(&next_token.item) {
            return returns
        }

        if is_arg_separator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            returns.push(create_error_node(expected_arg_separator_error(), next_token.position));  
            return returns;
        }
    }
}

fn parse_function_return_type(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let next_token = get_next_token(lexer);

    if let Some(return_type) = try_get_type(&next_token.item) {
        return create_node(create_type_item(return_type), next_token.position);
    }
    
    create_error_node(expected_type_error(), next_token.position)
}

fn parse_function_body(lexer: &mut Lexer) -> AbstractSyntaxNode {
    if !is_open_brace(&peek_next_token(lexer).item) {
        return create_error_node(expected_open_brace_error(), get_next_token(lexer).position);
    }

    let brace = get_next_token(lexer);
    let children = parse_function_body_nodes(lexer);
    
    assert!(is_close_brace(&peek_next_token(lexer).item));
    eat_next_token(lexer);

    create_node(create_function_body_item(children), brace.position)
}

fn parse_function_body_nodes(_lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    vec!()
}

fn parse_run_directive(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(create_run_directive_item(parse_next_node(lexer)), position)
}

fn parse_load_directive(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);
    if let Some(file_name) = try_get_string_literal(&token.item) {
        return create_node(create_load_directive_item(file_name), position);
    }
    create_error_node(expected_file_name_error(), token.position)
}

fn parse_literal(literal: Literal, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let node = create_node(create_literal_item(literal), position);
    
    if let SourceTokenItem::Operator(op) = peek_next_token(lexer).item {
        return parse_expression_item(lexer, op, node, position);
    }

    node
}

fn parse_expression_item(lexer: &mut Lexer, op: Operator, lhs: AbstractSyntaxNode, position: SourceFilePosition) -> AbstractSyntaxNode {
    eat_next_token(lexer);
    let rhs_node = parse_next_node(lexer);
    create_node(create_expression_item(op, lhs, rhs_node), position)
}

fn create_expression_item(op: Operator, lhs: AbstractSyntaxNode, rhs: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::BinaryExpr {
        op,
        lhs,
        rhs
    }
}

fn create_literal_item(literal: Literal) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Literal(literal)
}

fn create_run_directive_item(expr: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Run { expr }
}

fn create_constant_item(name: String, value: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Constant { name, value }
}

fn create_function_declaration_item(name: String, arguments: AbstractSyntaxChildNodes, return_types: AbstractSyntaxChildNodes, body: AbstractSyntaxNode) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::FunctionHeader { name, arguments, return_types, body }
}

fn create_arg_item(name: String, arg_type: Type) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::ArgumentDeclaration { name, arg_type }
}

fn create_type_item(t: Type) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Type(t)
}

fn create_load_directive_item(file_name: String) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Load { file_name }
}

fn create_function_body_item(children: AbstractSyntaxChildNodes) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::FunctionBody(children)
}

fn create_eof_item() -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Eof
}

fn create_node(item: AbstractSyntaxNodeItem, position: SourceFilePosition) -> AbstractSyntaxNode {
    AbstractSyntaxNode {
        item: Box::new(item),
        position,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AbstractSyntaxParseError {
    FileNotFoundError(String),
    ExpectedFileName,
    ExpectedOpenBrace,
    ExpectedArgName,
    ExpectedArgInitialise,
    ExpectedArgSeparator,
    ExpectedType,
    TokenisationError(SourceTokenError),
    Unimplemented
}

fn create_error_node(error: AbstractSyntaxParseError, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(AbstractSyntaxNodeItem::Error(error), position)
}

fn tokenisation_error(error: SourceTokenError) -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::TokenisationError(error)
}

fn unimplemented_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::Unimplemented
}

fn expected_open_brace_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedOpenBrace
}

fn expected_arg_name_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedArgName
}

fn expected_initialise_assignment_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedArgInitialise
}

fn expected_arg_separator_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedArgSeparator
}

fn expected_type_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedType
}

fn expected_file_name_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedFileName
}

fn file_not_found_error(file_name: String) -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::FileNotFoundError(file_name)
}

