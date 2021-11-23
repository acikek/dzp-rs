use std::env::current_dir;

use clap::ArgMatches;
use git2::Repository;

use crate::config::styles::read_style;
use crate::io::{log::err, fs::{create_dir, create}};
use crate::structs::project::ProjectData;

const MAIN: &str = r#"
  type: world
  events:
    after server start:
    - debug log "Hello, world!""#;

pub fn new(matches: &ArgMatches) {
    // Name of the project
    let name: String = matches.value_of("NAME").unwrap().to_string();
    // Style argument
    let style = matches.value_of("style").unwrap_or("default");
    // Forced to discontinue if unable to read style
    match read_style(style) {
        Ok(dirs) => {
            // The project data object
            // Take input if default argument not supplied
            let project = if matches.is_present("default") {
                Ok(ProjectData::from_name(name))
            } else {
                ProjectData::from_input(name)
            };
            
            if let Ok(project) = project {
                // Create style directories
                for dir in &dirs {
                    create_dir(dir.as_str(), true);
                }
                // Main dsc file
                create(
                    format!("{}/{}.dsc", dirs.last().unwrap(), &project.name).as_str(), 
                    format!("{}:{}", &project.name, MAIN));
                // README & LICENSE
                create("README.md", project.readme_header());
                create("LICENSE", project.license_text());
                // Create dzp directory
                create_dir(".dzp", true);
                create(".dzp/project", project.yaml());
                // Initialize git repository if possible
                match Repository::init(current_dir().unwrap()) {
                    Ok(_) => create(".gitignore", String::from("/.dzp")),
                    Err(e) => err("Failed to initialize git repository.", Some(e.to_string()))
                }
            }
        }
        Err(e) => err("Invalid style.", Some(e.to_string()))
    }
}