use clap::ArgMatches;
use serde_yaml::from_str;

use crate::io::log::err;
use crate::structs::project::ProjectData;

pub fn project(matches: &ArgMatches) {
    let path = matches.value_of("path").unwrap_or("./");

    match std::fs::read_to_string(path.to_owned() + ".dzp/project") {
        Ok(file) => {
            match from_str::<ProjectData>(&file) {
                Ok(data) => {
                    println!();
                    data.print();
                },
                Err(e) => err("Failed to parse project file.", Some(e.to_string()))
            }
        }
        Err(e) => err("Failed to read project file.", Some(e.to_string()))
    }
}