use crate::tokenisation::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Subtract
}

pub fn create_add_operator() -> Operator {
    Operator::Add
}

pub fn create_subtract_operator() -> Operator {
    Operator::Subtract
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum AssignmentOperator {
    Declaration,
    Initialise,
    InitialiseAssignValue,
    AssignValue,
    GoesTo
}

pub fn create_declaration_assignment_operator() -> AssignmentOperator {
    AssignmentOperator::Declaration
}

pub fn is_declaration_assignment(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Assignment(AssignmentOperator::Declaration)
}

pub fn is_initialise_assignment(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Assignment(AssignmentOperator::Initialise)
}

pub fn is_goes_to_assignment(item: &SourceTokenItem) -> bool {
    item == &SourceTokenItem::Assignment(AssignmentOperator::GoesTo)
}

pub fn create_initialise_assignment_operator() -> AssignmentOperator {
    AssignmentOperator::Initialise
}

pub fn create_initialise_assign_value_assignment_operator() -> AssignmentOperator {
    AssignmentOperator::InitialiseAssignValue
}

pub fn create_assign_value_assignment_operator() -> AssignmentOperator {
    AssignmentOperator::AssignValue
}

pub fn create_goes_to_assignment_operator() -> AssignmentOperator {
    AssignmentOperator::GoesTo
}




