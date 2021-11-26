use std::env::current_dir;

use clap::ArgMatches;
use git2::Repository;

use crate::config::styles::read_style;
use crate::io::{log::err, fs::{create_dir, create}};
use crate::structs::project::Project;

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
                Ok(Project::from_name(name))
            } else {
                Project::from_input(name)
            };
            
            if let Ok(project) = project {
                // Create style directories
                for dir in &dirs {
                    if dir != "." {
                        create_dir(dir.as_str(), true);
                    }
                }
                // Main dsc file
                create(
                    format!("{}/{}.dsc", dirs.last().unwrap(), &project.name).as_str(), 
                    format!("{}:{}", &project.name, MAIN), true);
                // README & LICENSE
                create("README.md", project.readme_header(), true);
                create("LICENSE", project.license_text(), true);
                // Create dzp directory
                create_dir(".dzp", true);
                create(".dzp/project", project.yaml(), true);
                // Initialize git repository if possible
                if let Err(e) = Repository::init(current_dir().unwrap()) {
                    err("Failed to initialize git repository.", Some(e.to_string()))
                }
            }
        }
        Err(e) => err("Invalid style.", Some(e.to_string()))
    }
}