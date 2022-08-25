#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Operator {
    Add,
}

pub fn create_add_operator() -> Operator {
    Operator::Add
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum AssignmentOperator {
    Declaration,
    Initialise,
    InitialiseAssignValue,
    AssignValue,
}

pub fn create_declaration_assignment_operator() -> AssignmentOperator {
    AssignmentOperator::Declaration
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

