use serde::{Serialize, Deserialize};
use swc_ecma_ast::{ImportDecl};
use std::{fs::{create_dir_all,File},path::{Path,PathBuf, MAIN_SEPARATOR}, ops::Deref};
use swc_ecma_visit::{swc_ecma_ast::Module, Visit, VisitWith, noop_visit_type};
use pathdiff::diff_paths;

// A single file's dependencies, as pure data
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Dependencies {
    local_files: Vec<PathBuf>,
    modules: Vec<String>,
}

pub fn extract_dependencies(base_path: &Path, input_path: &Path, module: Module) -> Dependencies {
    // Shouldn't panic since we know input_path points to a real file.
    let file_directory = input_path.parent().unwrap();

    let mut extractor = ImportExtractor{
        base_path: base_path.to_owned(),
        file_directory: file_directory.to_owned(),
        local_imports: Vec::new(),
        module_imports: Vec::new(),
    };
    module.visit_with(&mut extractor);

    // TODO: Implement this next.
    return Dependencies {
        local_files: extractor.local_imports,
        modules: extractor.module_imports,
    };
}

struct ImportExtractor{
    base_path: PathBuf,
    file_directory: PathBuf,
    local_imports: Vec<PathBuf>,
    module_imports: Vec<String>,
}
impl Visit for ImportExtractor {
    noop_visit_type!();

    fn visit_import_decl(&mut self, n: &ImportDecl) {
        let import = n.src.value.deref().to_owned();
        // Imports starting with . point to local files. Otherwise they're (probably) node_modules.
        // This may not hold for certain projects (e.g. using import aliases)
        if import.starts_with(".") {
            let normalized_import = import.replace("/", MAIN_SEPARATOR.to_string().as_str());
            let imported_path = self.file_directory.join(&normalized_import);
            let with_extension = find_file(imported_path);
            match with_extension {
                Some(file) => {
                    let relative_path = PathBuf::from(".").join(diff_paths(file.to_owned(), self.base_path.to_owned()).unwrap());
                    self.local_imports.push(relative_path)
                },
                None => eprintln!("ERROR: Couldn't resolve import {:?} from {:?}. Does this file exist?", import, self.file_directory),
            };
        } else {
          self.module_imports.push(import)
        }
    }
}

// A line like:
// 
//   import x from "./file" to read from "./file.ts"
//
// may refer to "./file.js", "./file.ts", "./file/index.js", etc.
// This function finds the match, if it exists. If not, the project
// probably doesn't compile.
fn find_file(path: PathBuf) -> Option<PathBuf> {
    if path.exists() {
        return Some(path)
    }
    let extensions = ["js", "ts", "jsx", "tsx"];
    for extension in extensions {
        let file = path.with_extension(extension);
        if file.exists() {
            return Some(file.canonicalize().unwrap());
        }
        let index = path.join("index");
        let index_file = index.with_extension(extension);
        if index_file.exists() {
            return Some(index_file.canonicalize().unwrap());
        }
    }

    None
}

pub fn write_dependencies(base_path: &Path, input: &Path, output_path: &Path, module: Module) {
    let dependencies = extract_dependencies(base_path, input, module);
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