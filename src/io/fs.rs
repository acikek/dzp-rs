use std::collections::BTreeMap;
use std::fs::{create_dir_all, write, read_to_string};

use serde_yaml::{from_str, Value};
use walkdir::WalkDir;

use super::log::err;

pub fn create_dir(path: &str, debug: bool) {
    if debug { println!("Creating {}...", path); }

    match create_dir_all(path) {
        Ok(_) => (),
        Err(e) => err("Failed to create directory.", Some(e.to_string()))
    }
}

pub fn create(path: &str, content: String) {
    println!("Creating {}...", path);

    match write(path, content) {
        Ok(_) => (),
        Err(e) => err("Failed to create file.", Some(e.to_string()))
    }
}

pub type ScriptContents = BTreeMap<String, Value>;
pub type ScriptFile = BTreeMap<String, ScriptContents>;

pub fn find_scripts() -> BTreeMap<String, (String, ScriptContents)> {
    // Get files in current directory
    let files = WalkDir::new(".").into_iter().filter_map(|e| e.ok());
    // File extension needs to be .dsc
    let script_files = files.filter(|f| f.path().extension().unwrap_or_default() == "dsc");
    // Return both the path string and the file contents
    let file_pairs = script_files.map(|f| {
        let path = String::from(f.clone().path().to_string_lossy());
        (path, read_to_string(f.path()))
    })
    .filter(|r| r.1.is_ok());
    // Parse file contents
    let parsed_files = file_pairs.map(|(p, c)| (p, from_str::<ScriptFile>(&c.unwrap())))
        // Validate parse
        .filter(|r| r.1.is_ok())
        .map(|(p, c)| (p, c.unwrap()))
        .collect::<Vec<(String, ScriptFile)>>();

    let mut result = BTreeMap::<String, (String, ScriptContents)>::new();

    for (path, scripts) in parsed_files {
        for (script, contents) in scripts {
            result.insert(script, (path.clone(), contents));
        }
    }

    result
}