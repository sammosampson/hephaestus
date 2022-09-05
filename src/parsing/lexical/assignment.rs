use crate::parsing::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Assignment {
    Declaration,
    Initialise,
    InitialiseAssignValue,
    AssignValue,
    GoesTo
}

pub fn create_assignment_token_item(op: Assignment) -> SourceTokenItem {
    SourceTokenItem::Assignment(op)
}

pub fn create_declaration_assignment() -> Assignment {
    Assignment::Declaration
}

pub fn is_declaration_assignment(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Assignment(Assignment::Declaration)
}

pub fn is_initialise_assignment(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Assignment(Assignment::Initialise)
}

pub fn is_initialise_assign_value_assignment(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Assignment(Assignment::InitialiseAssignValue)
}

pub fn is_goes_to_assignment(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Assignment(Assignment::GoesTo)
}

pub fn create_initialise_assignment() -> Assignment {
    Assignment::Initialise
}

pub fn create_initialise_assign_value_assignment() -> Assignment {
    Assignment::InitialiseAssignValue
}

pub fn create_assign_value_assignment() -> Assignment {
    Assignment::AssignValue
}

pub fn create_goes_to_assignment() -> Assignment {
    Assignment::GoesTo
}




