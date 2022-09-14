use crate::{
    running::*
};

#[derive(Clone)]
pub struct TestInterpreter; 

impl Interpret for TestInterpreter {
    fn interpret(&mut self, _code: RunnableCompileTimeCode) {
    }
}

pub fn create_test_interpreter() -> TestInterpreter {
    TestInterpreter
}