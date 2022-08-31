use crate::file_system::*;
use crate::parsing::*;

pub type AbstractSyntaxChildNodeItem = Box<AbstractSyntaxNodeItem>;
pub type AbstractSyntaxChildNodes = Vec<AbstractSyntaxNode>;

#[derive(PartialEq, Debug)]
pub struct AbstractSyntaxTree {
    pub children: AbstractSyntaxChildNodes,
}

fn create_tree(children: Vec<AbstractSyntaxNode>) -> AbstractSyntaxTree {
    AbstractSyntaxTree { children }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AbstractSyntaxNode {
    pub position: SourceFilePosition,
    pub item: AbstractSyntaxChildNodeItem,
}

impl AbstractSyntaxNode {
    pub fn item_ref(&self) -> &AbstractSyntaxNodeItem {
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

pub fn parse_file(file_name: &str) -> AbstractSyntaxTree {
    match read_file_to_string(file_name) {
        Ok(file_content) => parse(&file_content),
        Err(_) => create_tree(vec!(create_error_node(file_not_found_error(file_name.to_string()), empty_position())))
    }
}

pub fn parse(input: &str) -> AbstractSyntaxTree {
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

pub fn parse_next_node(lexer: &mut Lexer) -> AbstractSyntaxNode {
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

fn create_eof_item() -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Eof
}

pub fn create_node(item: AbstractSyntaxNodeItem, position: SourceFilePosition) -> AbstractSyntaxNode {
    AbstractSyntaxNode {
        item: Box::new(item),
        position,
    }
}

