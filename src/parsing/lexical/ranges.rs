#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Range {
    LeftInclusive,
}

pub fn create_left_inclusive_range() -> Range {
    Range::LeftInclusive
}