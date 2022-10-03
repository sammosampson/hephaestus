use crate::parsing::*;
use crate::typing::*;

pub type AbstractSyntaxChildNodeItem = Box<AbstractSyntaxNodeItem>;
pub type AbstractSyntaxChildNodes = Vec<AbstractSyntaxNode>;

#[derive(PartialEq, Debug, Clone)]
pub struct AbstractSyntaxNode {
    pub position: SourceFilePosition,
    pub item: AbstractSyntaxChildNodeItem,
}

impl AbstractSyntaxNode {
    pub fn item_ref(&self) -> &AbstractSyntaxNodeItem {
        self.item.as_ref()
    }

    pub fn item_mut(&mut self) -> &mut AbstractSyntaxNodeItem {
        self.item.as_mut()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AbstractSyntaxNodeItem {
    Run { expr: AbstractSyntaxNode },
    Load { file: AbstractSyntaxNode },
    ForeignSystemLibrary { library: AbstractSyntaxNode },
    ProcedureHeader {
        name: String,
        args: AbstractSyntaxChildNodes,
        return_args: AbstractSyntaxChildNodes,
        body: ProcedureBodyReference
    },
    ProcedureBody { 
        name: String,
        args: AbstractSyntaxChildNodes,
        return_types: AbstractSyntaxChildNodes,
        statements: AbstractSyntaxChildNodes,
    },
    ProcedureCall {
        name: String,
        args: AbstractSyntaxChildNodes,
        procedure_call_type: ResolvableType
    },
    ArgumentDeclaration {
        name: String,
        arg_type: ResolvableType
    },
    Argument {
        expr: AbstractSyntaxNode,
        arg_type: ResolvableType
    },
    Null,
    Return {
        args: AbstractSyntaxChildNodes
    },
    Constant {
        name: String,
        value: AbstractSyntaxNode
    },
    Assignment {
        name: String,
        value: AbstractSyntaxNode,
        assignment_type: ResolvableType
    },
    BinaryExpr {
        op: AbstractSyntaxNode,
        lhs: AbstractSyntaxNode,
        rhs: AbstractSyntaxNode,
        expression_type: ResolvableType,
    },
    Type(ResolvableType),
    Literal(ResolvableLiteral),
    Identifier(String),
    Operator(Operator),
    Error(ParseError),
    Eof
}