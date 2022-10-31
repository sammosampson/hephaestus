mod directives;
mod consts;
mod procedures;
mod expressions;
mod assignments;
mod structs;

use rust_hephaestus::*;

#[test]
fn parse_empty_input_parses_correctly() {
    let file_path = "test.hep";
    
    let (actual_file_path, units, ..) = run_parse_file(
        file_path, 
        ""
    );
       
    assert_eq!(actual_file_path, file_path.to_string());
    assert_eq!(0, units.len());
}