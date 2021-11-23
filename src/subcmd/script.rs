use clap::ArgMatches;
use lazer::lazer;
use serde_yaml::from_value;

use crate::io::log::err; 
use crate::io::fs::{find_scripts, ScriptContents};

pub fn print_script(name: &str, path: &String, data: &ScriptContents) {
    if !data.contains_key("type") {
        err("Script does not contain a 'type' key.", None);
    } else {
        match from_value::<String>(data.get("type").unwrap().clone()) {
            Ok(script_type) => {
                lazer()
                    .print_ln(&format!("Name: {}", name))
                    .print_ln(&format!("Type: {}", script_type))
                    .print_ln(&format!("Path: {}", path))
                    .print_ln(&format!("Keys: {}", data.keys().map(|s| s.clone()).collect::<Vec<String>>().join(", ")));
            }
            Err(e) => err("Failed to parse 'type' key.", Some(e.to_string()))
        }
    }


}

pub fn script(matches: &ArgMatches) {
    let scripts = find_scripts();

    match matches.value_of("NAME") {
        Some(name) => {
            match scripts.get(name) {
                Some((path, data)) => {
                    println!();
                    print_script(name, &path, &data);
                }
                None => err("Failed to find script.", None)
            }
        },
        None => {
            let keys = scripts.keys()
                .clone()
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
                .join(", ");

            println!("{}", keys);
        }
    }
}