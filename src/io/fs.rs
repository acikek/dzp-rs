use std::fs::{create_dir_all, write};

use super::log::err;

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