#[cfg(test)]
mod tests;

mod parsing;
mod typing;
mod intermediate_representation;
mod running;
mod file_system;
mod arguments;
mod threading;
mod compilation;
mod collections;
mod acting;
mod utilities;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    
    match arguments::get_file_to_compile_from_invocation_arguments() {
        Some(file_name) => { 
            compilation::compile(
                file_name, 
                file_system::create_file_reader(), 
                running::create_interpreter(),
                compilation::create_null_message_wire_tap()
            );
        },
        None => panic!("No compilation file name argument passed")
    }
}

