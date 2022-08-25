use crate::{
    tokenisation::*,
    source_files::*,
    literals::*,
    operators::*, 
    directives::*,
    file_system::*
};

#[test]
fn parse_empty_input() {
    let ast = parse("");
    assert_eq!(1, ast.children.len());
    let child = &ast.children[0];
    assert_eq!(0, child.position.absolute);
    assert_eq!(0, child.position.col);
    assert_eq!(0, child.position.line);
    assert_eq!(child.item_ref().clone(), AbstractSyntaxNodeItem::Eof);
}

#[test]
fn parse_run_expression() {
    let ast = parse("#run 1 + 2");
    assert_eq!(2, ast.children.len());
    
    let child = &ast.children[0];
    assert_eq!(0, child.position.absolute);
    assert_eq!(1, child.position.col);
    assert_eq!(1, child.position.line);
    
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
fn parse_load_expression() {
    let ast = parse("#load \"test.jai\"");
    assert_eq!(2, ast.children.len());
    
    let child = &ast.children[0];
    assert_eq!(0, child.position.absolute);
    assert_eq!(1, child.position.col);
    assert_eq!(1, child.position.line);
    
    match ast.children[0].item_ref() {
        AbstractSyntaxNodeItem::Load { file_name } => assert_eq!(file_name, "test.jai"),
        _ => crate::testing::assert_fail("Load not returned"),
    }
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

fn parse_run_directive(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(create_run_directive_item(parse_next_node(lexer)), position)
}

fn parse_load_directive(lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);
    return match token.item {
        SourceTokenItem::Literal(literal) => match literal {
            Literal::String(file_name) => create_node(create_load_directive_item(file_name), position),
            _ => create_error_node(expected_file_name_error(), token.position)
        },
        _ => create_error_node(expected_file_name_error(), token.position)
    }
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

fn create_load_directive_item(file_name: String) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Load { file_name }
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

fn expected_file_name_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedFileName
}

fn file_not_found_error(file_name: String) -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::FileNotFoundError(file_name)
}

