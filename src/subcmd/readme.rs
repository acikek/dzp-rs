use crate::io::fs::{create, get_project};

pub fn readme() {
    if let Ok(project) = get_project(None) {
        create("README.md", project.readme(), true);
    }
}