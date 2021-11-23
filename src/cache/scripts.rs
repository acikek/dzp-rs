use std::collections::BTreeMap;
use std::fs::{read_to_string, write};
use std::path::Path;

use serde_yaml::{from_str, Value, to_string};
use walkdir::WalkDir;

use crate::io::log::err;
use crate::structs::script::Script;

pub type ScriptContents = BTreeMap<String, Value>;
pub type ScriptFile = BTreeMap<String, ScriptContents>;

pub fn find_scripts() -> BTreeMap<String, Script> {
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

    let mut result = BTreeMap::<String, Script>::new();

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

pub fn get_scripts() -> BTreeMap<String, Script> {
    match read_to_string("./.dzp/scripts") {
        Ok(cache) => {
            match from_str::<BTreeMap<String, Script>>(&cache) {
                Ok(data) => data,
                Err(_) => BTreeMap::new()
            }
        }
        Err(_) => {
            let result = find_scripts();

            if Path::new("./.dzp").exists() {
                let ser = to_string(&result);
                if let Ok(content) = ser {
                    let _ = write("./.dzp/scripts", content);
                }
            }
            
            result
        }
    }
}