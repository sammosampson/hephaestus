use rust_hephaestus::*;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    
    match get_file_to_compile_from_invocation_arguments() {
        Some(file_name) => { 
            compile(
                file_name, 
                create_file_reader(), 
                create_x64_backend(),
                create_null_message_wire_tap()
            );
        },
        None => panic!("No compilation file name argument passed")
    }
}

