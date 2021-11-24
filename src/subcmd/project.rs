use clap::ArgMatches;

use crate::io::fs::get_project;

pub fn project(matches: &ArgMatches) {
    let path = matches.value_of("path");

    if let Ok(project) = get_project(path) {
        println!();
        project.print();
    }
}