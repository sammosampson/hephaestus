use crate::parsing::*;

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
        arguments: AbstractSyntaxChildNodes,
        return_types: AbstractSyntaxChildNodes,
        body: CompilationUnitReference
    },
    ProcedureBody(AbstractSyntaxChildNodes),
    ProcedureCall {
        name: String,
        arguments: AbstractSyntaxChildNodes,
        arg_type: Type
    },
    ArgumentDeclaration { name: String, arg_type: Type },
    Argument { expr: AbstractSyntaxNode, arg_type: Type },
    Type(Type),
    Constant {
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