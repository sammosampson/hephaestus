mod tokenisation;
mod abstract_syntax;
mod source_files;
mod operators;
mod literals;
mod keywords;
mod ranges;
mod enclosures;
mod terminators;
mod directives;
mod file_system;
mod arguments;
mod testing;

use crate::abstract_syntax::*;

fn main() {
    match arguments::get_file_to_compile_from_invocation_arguments() {
        Some(file_name) => { dbg!(parse_file(file_name)); },
        None => panic!("No compilation file name argument passed")
    }
}
