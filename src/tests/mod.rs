pub mod parsing;
pub mod lexical;

pub fn assert_fail(failure_text: &str) {
    panic!("asertion failed: {failure_text}")
}