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
    Load { file_name: String },
    ProcedureHeader {
        name: String,
        args: AbstractSyntaxChildNodes,
        return_types: AbstractSyntaxChildNodes,
        body: CompilationUnitReference
    },
    ProcedureBody { 
        args: AbstractSyntaxChildNodes,
        return_types: AbstractSyntaxChildNodes,
        statements: AbstractSyntaxChildNodes,
    },
    ProcedureCall {
        name: String,
        args: AbstractSyntaxChildNodes,
        type_id: ResolvableType,
        return_type_ids: ResolvableTypes,
    },
    ArgumentDeclaration {
        name: String,
        type_id: ResolvableType
    },
    Argument {
        expr: AbstractSyntaxNode,
        type_id: ResolvableType
    },
    Constant {
        name: String,
        value: AbstractSyntaxNode
    },
    Assignment {
        name: String,
        value: AbstractSyntaxNode
    },
    BinaryExpr {
        op: AbstractSyntaxNode,
        lhs: AbstractSyntaxNode,
        rhs: AbstractSyntaxNode,
    },
    Type(ResolvableType),
    Literal(Literal),
    Identifier(String),
    Operator(Operator),
    Error(ParseError),
    Eof
}

pub trait AbstractSyntaxRootNodeVisitor {
    fn visit_run(&mut self, expr: &mut AbstractSyntaxNode);

    fn visit_procedure_header(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        return_types: &mut AbstractSyntaxChildNodes,
        body: &mut CompilationUnitReference
    );

    fn visit_procedure_body(
        &mut self, 
        args: &mut AbstractSyntaxChildNodes,
        return_types: &mut AbstractSyntaxChildNodes,
        statements: &mut AbstractSyntaxChildNodes
    );
}

pub trait AbstractSyntaxProcedureHeaderNodeVisitor {
    fn visit_argument_declaration(&mut self, name: &mut String, type_id: &mut ResolvableType);
    fn visit_return_type_declaration(&mut self, return_type: &mut ResolvableType);
}

pub trait AbstractSyntaxProcedureBodyNodeVisitor {
    fn visit_procedure_call(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        type_id: &mut ResolvableType,
        return_type_ids: &mut ResolvableTypes,
    );
    
    fn visit_assignment(&mut self, name: &mut String, value: &mut AbstractSyntaxNode);
}

pub trait AbstractSyntaxProcedureCallNodeVisitor {
    fn visit_argument(&mut self, expr: &mut AbstractSyntaxNode, type_id: &mut ResolvableType);
}

pub trait AbstractSyntaxExpressionNodeVisitor {
    fn visit_literal(&mut self, literal: &mut Literal);
    fn visit_procedure_call(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        type_id: &mut ResolvableType,
        return_type_ids: &mut ResolvableTypes,
    );
}

pub fn apply_visitor_to_ast_root<TVistor>(ast: &mut AbstractSyntaxNode, visitor: &mut TVistor) 
where TVistor : AbstractSyntaxRootNodeVisitor {
    match ast.item_mut() {
        AbstractSyntaxNodeItem::Run { expr } =>
            visitor.visit_run(expr),
        AbstractSyntaxNodeItem::ProcedureHeader { 
            name,
            args,
            return_types,
            body
        } => visitor.visit_procedure_header(name, args, return_types, body),
        AbstractSyntaxNodeItem::ProcedureBody { 
            args,
            return_types,
            statements 
        } => {
            visitor.visit_procedure_body(args, return_types, statements);
        },
        item => handle_cannot_visit_node(item)
    }
}

pub fn apply_visitor_to_ast_procedure_header<TVistor>(
    args: &mut AbstractSyntaxChildNodes,
    return_types: &mut AbstractSyntaxChildNodes,
    visitor: &mut TVistor
) 
where TVistor : AbstractSyntaxProcedureHeaderNodeVisitor {
    for arg in args {
        match arg.item_mut() {
            AbstractSyntaxNodeItem::ArgumentDeclaration { name, type_id } => 
                visitor.visit_argument_declaration(name, type_id),
            item => handle_cannot_visit_node(item)
        }
    }

    for return_type in return_types {
        match return_type.item_mut() {
            AbstractSyntaxNodeItem::Type(resolvable_type) => 
                visitor.visit_return_type_declaration(resolvable_type),
            item => handle_cannot_visit_node(item)
        }
    }
}

pub fn apply_visitor_to_ast_procedure_body<TVistor>(statements: &mut AbstractSyntaxChildNodes, visitor: &mut TVistor) 
where TVistor : AbstractSyntaxProcedureBodyNodeVisitor {
    for statement in statements {
        match statement.item_mut() {
            AbstractSyntaxNodeItem::ProcedureCall { 
                name, 
                args, 
                type_id, 
                return_type_ids
            } => 
                visitor.visit_procedure_call(name, args, type_id, return_type_ids),
            AbstractSyntaxNodeItem::Assignment { name, value } => 
                visitor.visit_assignment(name, value),
            item => handle_cannot_visit_node(item)
        }
    }
}

pub fn apply_visitor_to_ast_procedure_call<TVistor>(args: &mut AbstractSyntaxChildNodes, visitor: &mut TVistor) 
where TVistor : AbstractSyntaxProcedureCallNodeVisitor {
    for arg in args {
        match arg.item_mut() {
            AbstractSyntaxNodeItem::Argument { expr, type_id } => 
                visitor.visit_argument(expr, type_id),
            item => handle_cannot_visit_node(item)
        }
    }
}

pub fn apply_visitor_to_ast_expression<TVistor>(ast: &mut AbstractSyntaxNode, visitor: &mut TVistor) 
where TVistor : AbstractSyntaxExpressionNodeVisitor {
    match ast.item_mut() {
        AbstractSyntaxNodeItem::Literal(literal) => visitor.visit_literal(literal),
        AbstractSyntaxNodeItem::ProcedureCall {
            name, 
            args, 
            type_id,
            return_type_ids
        } => 
                visitor.visit_procedure_call(name, args, type_id, return_type_ids),
        item => handle_cannot_visit_node(item)
    }
}

fn handle_cannot_visit_node(item: &AbstractSyntaxNodeItem) {
    panic!("cannot visit node at this level {:?}", item)
}
