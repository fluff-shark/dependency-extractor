use serde::{Serialize, Deserialize};
use std::{fs::{create_dir_all,File},path::{Path,PathBuf}};
use swc_ecma_visit::{swc_ecma_ast::Module,Visit};

// A single file's dependencies, as pure data
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Dependencies {
    dependencies: Vec<PathBuf>,
}

pub fn extract_dependencies(module: Module) -> Dependencies {
    // TODO: Implement this next.
    return Dependencies {
        dependencies: Vec::new(),
    };
}

struct ImportExtractor{}
impl Visit for ImportExtractor {

}

pub fn write_dependencies(output_path: &Path, module: Module) {
    let dependencies = extract_dependencies(module);
    let parent_directory = output_path.parent().unwrap();
    match create_dir_all(parent_directory) {
        Ok(_) => (),
        Err(e) => panic!("Failed to create output directory: {:?}", e)
    };
    let file = match File::create(output_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error creating file: {:?}", e);
            return;
        }
    };

    match serde_yaml::to_writer(file, &dependencies) {
        Ok(_) => (),
        Err(e) => panic!("Failed to serialize {:?} to file {:?}: {:?}", dependencies, output_path, e)
    };
}