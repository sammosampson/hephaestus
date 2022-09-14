use crate::{
    running::*
};

pub trait Interpret : Send + Clone + 'static {
    fn interpret(&mut self, code: RunnableCompileTimeCode);
}

#[derive(Clone)]
pub struct Interpreter;

impl Interpret for Interpreter {
    fn interpret(&mut self, code: RunnableCompileTimeCode) {
        dbg!(code);
    }
}

pub fn create_interpreter() -> Interpreter {
    Interpreter
}