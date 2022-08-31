mod tests;
mod parsing;
mod file_system;
mod arguments;
mod threading;
mod compilation;
mod collections;

fn main() {
    match arguments::get_file_to_compile_from_invocation_arguments() {
        Some(file_name) => { compilation::compile(file_name); },
        None => panic!("No compilation file name argument passed")
    }
}