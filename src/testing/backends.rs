use crate::intermediate_representation::*;
use crate::backends::*;

#[derive(Clone)]
pub struct TestBackend; 

impl BackendBuild for TestBackend {
    fn build_backend(&mut self, _ir: IntermediateRepresentation) -> BackendErrorResult {
        Ok(())
    }
}

pub fn create_test_backend() -> TestBackend {
    TestBackend
}