use crate::parsing::*;

pub fn node(position: SourceFilePosition, item: AbstractSyntaxNodeItem) -> AbstractSyntaxNode {
    create_node(item, position)
}

pub fn position(absolute: usize, line: usize, col: usize) -> SourceFilePosition {
    create_source_file_position(absolute, line, col)
}
