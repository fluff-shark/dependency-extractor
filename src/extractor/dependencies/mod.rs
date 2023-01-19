use std::path::Path;

mod output;
mod parse;

pub fn log_dependencies(base_path: &Path, input: &Path, output_path: &Path) {
     match parse::parse_module(input) {
        Ok(module) => output::write_dependencies(base_path, input, output_path, module),
        Err(e) => {
            e.io_error.iter().for_each(|err| panic!("failed to read {}: {:?}", input.display(), err));
        }
     }
}
