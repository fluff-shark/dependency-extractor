use assert_cmd::prelude::*;
use globwalk::GlobWalkerBuilder;
use pathdiff::diff_paths;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fs::{File, read_dir, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;

const EXPECTED_WORKSPACE_FOLDER: &str = ".dependencies";

// Run the script on each "cases/{case}/project" and assert that
// cases/*/project/.dependencies matches cases/{case}/expectations.
#[test]
fn integration_tests() -> Result<(), Box<dyn std::error::Error>> {
    let cases = read_dir("./tests/cases")?;
    for case in cases {
        integration_test(case?.path())?;
    }
    return Ok(())
}

// Make sure a single "cases/{case}" directory works properly.
fn integration_test(case: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let project = case.join("project");
    cleanup_old_dependencies(project.as_path())?;
    execute_cli_on(project.as_path())?;
    assert_directories_match(&case, project.join(EXPECTED_WORKSPACE_FOLDER), case.join("expectations"))?;
    return Ok(());
}

// Assert that:
//   1. The same directories & files exist in each of the two paths.
//   2. The contents of all the .yaml files match semantically.
fn assert_directories_match(case: &PathBuf, actual: PathBuf, expected: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let walker = GlobWalkerBuilder::from_patterns(expected.canonicalize()?, &["**/*.yaml"]).build()?;
    for file_result in walker {
        let file = file_result?;
        let relative_path = PathBuf::from(".").join(diff_paths(file.path(), expected.canonicalize()?).unwrap());
        let actual_file = actual.join(&relative_path);
        assert!(actual_file.exists());
        assert_contents_match(case.join("expectations").join(&relative_path), actual_file)?;
    }
    return Ok(());
}

// Make sure that the .yaml contents of two files match semantically.
fn assert_contents_match(expected_path: PathBuf, actual_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let expected = parse_contents(expected_path)?;
    let actual = parse_contents(actual_path)?;
    assert_eq!(expected, actual);
    return Ok(());
}

// Parse a .yaml file from .dependencies into a struct for better comparisons.
fn parse_contents(path: PathBuf) -> Result<OutputFileFormat, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let parsed: OutputFileFormat = serde_yaml::from_reader(file)?;
    return Ok(parsed);
}

// Clean old .dependencies folders which may hang around from previous runs. 
fn cleanup_old_dependencies(project: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let existing_dependencies = project.join(EXPECTED_WORKSPACE_FOLDER);
    if existing_dependencies.exists() {
        remove_dir_all(project.join(EXPECTED_WORKSPACE_FOLDER))?;
    }
    return Ok(());
}

// Execute "cargo run -- -b ./tests/cases/module/project"
fn execute_cli_on(project: &Path) -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("-b")
        .arg(project.display().to_string())
        .assert()
        .success();
    return Ok(());
}

// Contract class for the .yaml output files.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputFileFormat {
    local_files: HashSet<PathBuf>,
    modules: HashSet<String>,
}
