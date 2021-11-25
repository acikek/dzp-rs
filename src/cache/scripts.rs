use std::collections::BTreeMap;
use std::fs::{read_to_string, write};
use std::path::Path;

use serde_yaml::{from_str, Value, to_string};
use walkdir::WalkDir;

use crate::io::log::err;
use crate::structs::script::Script;

pub type ScriptContents = BTreeMap<String, Value>;
pub type ScriptFile = BTreeMap<String, ScriptContents>;

pub fn find_scripts_raw() -> Vec<(String, ScriptFile)> {
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
    // dzp ignore rule
    let unignored = file_pairs
        .map(|(p, r)| (p, r.unwrap()))
        .filter(|r| !r.1.replace(" ", "").starts_with("#:ignore"));
    // Parse file contents
    let parsed_files = unignored.map(|(p, c)| (p, from_str::<ScriptFile>(&c)))
        // Validate parse
        .filter(|r| r.1.is_ok())
        .map(|(p, c)| (p, c.unwrap()))
        .collect::<Vec<(String, ScriptFile)>>();

    let mut result = Vec::<(String, ScriptFile)>::new();

    for (path, file) in parsed_files {
        for (name, contents) in file {
            let mut map = BTreeMap::new();
            map.insert(name, contents);
            result.push((path.clone(), map));
        }
    }

    result
}

pub fn find_scripts() -> BTreeMap<String, Script> {
    let parsed_files = find_scripts_raw();
    let mut result = BTreeMap::new();

    for (path, scripts) in parsed_files {
        for (name, contents) in scripts {
            let s = Script::from((path.clone(), contents));
            match s {
                Ok(script) => { let _ = result.insert(name, script); },
                Err(e) => err(&e, None)
            }
        }
    }

    result
}

pub fn script_contents() -> Vec<String> {
    let scripts = find_scripts_raw();
    let mut unique = Vec::new();

    for file in &scripts {
        let keys = file.1.keys().map(|s| s.clone()).collect::<Vec<String>>();
        let script_name = keys.first().unwrap();

        if !scripts.iter().find(|(name, _)| name == script_name).is_some() {
            unique.push(file);
        }
    }
    
    unique
        .iter()
        .map(|(_, script)| to_string(script))
        .filter_map(|s| s.ok())
        .map(|s| s.trim_start_matches("---\n")
            // Hack to remove escaped quotes from commands
            .replace("\"", "")
            .replace("\\", "\"")
            .to_owned())
        .collect::<Vec<String>>()
}

pub fn write_scripts() -> BTreeMap<String, Script> {
    let result = find_scripts();

    if Path::new("./.dzp").exists() {
        let ser = to_string(&result);
        if let Ok(content) = ser {
            let _ = write("./.dzp/scripts", content);
        }
    }
    
    result
}

pub fn get_scripts(force: bool) -> BTreeMap<String, Script> {
    if force {
        write_scripts()
    } else {
        match read_to_string("./.dzp/scripts") {
            Ok(cache) => {
                match from_str::<BTreeMap<String, Script>>(&cache) {
                    Ok(data) => data,
                    Err(_) => BTreeMap::new()
                }
            }
            Err(_) => {
                write_scripts()
            }
        }
    }
}

pub fn get_script_types() -> BTreeMap<String, Vec<String>> {
    let scripts = get_scripts(true);
    let mut result = BTreeMap::<String, Vec<String>>::new();

    for (name, script) in scripts {
        match result.get_mut(&script.script_type) {
            Some(v) => v.push(name),
            None => { let _ = result.insert(script.script_type, vec![name]); }
        }
    }

    result
}