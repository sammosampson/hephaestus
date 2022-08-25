use std::env;

pub fn get_file_to_compile_from_invocation_arguments() -> Option<String>{
    let args: Vec<String> = env::args().collect();
    if let Some(file_name) = args.get(1) {
        return Some(file_name.clone())
    }
    None
}