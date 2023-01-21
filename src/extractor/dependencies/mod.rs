use std::path::Path;

mod output;
mod parse;

pub fn log_dependencies(base_path: &Path, input: &Path, output_path: &Path)
    -> Result<(), Box<dyn std::error::Error>>
{
    let module = parse::parse_module(input)?;
    return output::write_dependencies(base_path, input, output_path, module);
}
