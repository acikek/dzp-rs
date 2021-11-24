use clap::ArgMatches;

use crate::cache::scripts::script_contents;
use crate::io::fs::{create, get_project};

pub fn compile(matches: &ArgMatches) {
    let scripts = script_contents().join("\n");

    let file = match matches.value_of("FILE") {
        Some(file) => file.to_owned(),
        None => {
            if let Ok(project) = get_project(None) {
                project.name + ".dsc"
            } else {
                "compiled.dsc".to_owned()
            }
        }
    };

    create(&file, scripts, true)
}