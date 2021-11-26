use std::fs::{create_dir_all, read_to_string, write};

use serde_yaml::from_str;

use super::log::err;
use crate::structs::project::Project;

pub fn debug_path(path: &str, debug: bool) {
    if debug {
        println!("Creating {}...", path);
    }
}

pub fn create_dir(path: &str, debug: bool) {
    debug_path(path, debug);

    match create_dir_all(path) {
        Ok(_) => (),
        Err(e) => err("Failed to create directory.", Some(e.to_string()))
    }
}

pub fn create(path: &str, content: String, debug: bool) {
    debug_path(path, debug);

    match write(path, content) {
        Ok(_) => (),
        Err(e) => err("Failed to create file.", Some(e.to_string()))
    }
}

pub fn get_project(path: Option<&str>) -> Result<Project, ()> {
    match read_to_string(path.unwrap_or("").to_owned() + ".dzp/project") {
        Ok(file) => {
            match from_str::<Project>(&file) {
                Ok(data) => Ok(data),
                Err(e) => { err("Failed to parse project file.", Some(e.to_string())); Err(()) }
            }
        }
        Err(e) => { err("Failed to read project file.", Some(e.to_string())); Err(()) }
    }
}