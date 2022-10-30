use uuid::*;
use crate::parsing::*;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct CompilationUnitId {
    id: Uuid
}

impl std::fmt::Debug for CompilationUnitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id.to_string())
    }
}

pub fn create_compilation_unit_id() -> CompilationUnitId {
    CompilationUnitId {
        id: uuid::Uuid::new_v4()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CompilationUnit {
    pub id: CompilationUnitId,
    pub filename: String,
    pub tree: AbstractSyntaxNode
}

pub fn create_unit(filename: String, tree: AbstractSyntaxNode) -> CompilationUnit {
    CompilationUnit {
        id: create_compilation_unit_id(),
        filename,
        tree
    }
}

pub type CompilationUnits = Vec<CompilationUnit>;
