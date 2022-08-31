use uuid::*;
use crate::parsing::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct CompilationUnitId {
    id: Uuid
}

pub fn create_compilation_unit_id() -> CompilationUnitId {
    CompilationUnitId {
        id: uuid::Uuid::new_v4()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CompilationUnit {
    pub tree: AbstractSyntaxNode,
    pub id: CompilationUnitId
}

pub fn create_unit(tree: AbstractSyntaxNode) -> CompilationUnit {
    CompilationUnit {
        tree,
        id: create_compilation_unit_id()
    }
}

pub type CompilationUnits = Vec<CompilationUnit>;


#[derive(PartialEq, Debug, Clone)]
pub enum CompilationUnitReference {
    Resolved(CompilationUnitId),
}