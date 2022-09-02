mod directives;
mod consts;
mod procedures;

use crate::parsing::*;

#[test]
fn parse_empty_input_parses_correctly() {
    let ast = parse("");
    assert_eq!(0, ast.len());
}