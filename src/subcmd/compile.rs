use clap::ArgMatches;

use crate::cache::scripts::script_contents;
use crate::io::fs::{create, get_project};

pub fn compile(matches: &ArgMatches) {
    let scripts = script_contents().join("\n");
    let mut header = String::new();

    let file = match matches.value_of("FILE") {
        Some(file) => file.to_owned(),
        None => {
            if let Ok(project) = get_project(None) {
                header = project.comment_header() + "\n\n";
                project.name + ".dsc"
            } else {
                "compiled.dsc".to_owned()
            }
        }
    };

    let content = format!("#: ignore\n\n{}{}", header, scripts);
    create(&file, content, true)
}