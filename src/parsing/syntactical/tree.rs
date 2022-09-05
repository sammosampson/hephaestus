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
}

#[derive(PartialEq, Debug, Clone)]
pub enum AbstractSyntaxNodeItem {
    Run { expr: AbstractSyntaxNode },
    Load { file_name: String },
    ProcedureHeader {
        name: String,
        args: AbstractSyntaxChildNodes,
        return_types: AbstractSyntaxChildNodes,
        body: CompilationUnitReference
    },
    ProcedureBody(AbstractSyntaxChildNodes),
    ProcedureCall {
        name: String,
        args: AbstractSyntaxChildNodes,
        arg_type: ResolvableType
    },
    ArgumentDeclaration { name: String, arg_type: ResolvableType },
    Argument { expr: AbstractSyntaxNode, arg_type: ResolvableType },
    Type(ResolvableType),
    Constant {
        name: String,
        value: AbstractSyntaxNode
    },
    Assignment {
        name: String,
        value: AbstractSyntaxNode
    },
    Literal(Literal),
    Identifier(String),
    BinaryExpr {
        op: Operator,
        lhs: AbstractSyntaxNode,
        rhs: AbstractSyntaxNode,
    },
    Error(ParseError),
    Eof
}