#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct SourceFilePosition {
    pub absolute: usize,
    pub line: usize,
    pub col: usize,
}

pub fn create_source_file_position(absolute: usize, line: usize, col: usize) -> SourceFilePosition {
    SourceFilePosition { absolute, line, col }
}

pub fn empty_position() -> SourceFilePosition {
    SourceFilePosition::default()
}

pub fn first_character_in_source_file_position() -> SourceFilePosition {
    create_source_file_position(0, 1, 1)
}

pub fn increment_source_file_position_col(position: SourceFilePosition) -> SourceFilePosition {
    create_source_file_position(position.absolute + 1, position.line, position.col + 1)
}

pub fn increment_source_file_position_line(position: SourceFilePosition) -> SourceFilePosition {
    create_source_file_position(position.absolute + 1, position.line + 1, 0)
}