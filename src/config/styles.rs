use std::fs::{read_to_string, write};
use std::path::Path;

use dirs::config_dir;

use crate::io::fs::create_dir;

pub fn get_dir() -> String {
    String::from(config_dir().unwrap().to_str().unwrap())
}

pub fn init_styles(force: bool) {
    let dir = format!("{}/dzp_styles", get_dir());

    if force || !Path::new(&dir).exists() {
        create_dir(dir.as_str(), false);
        let _ = write(format!("{}/default.txt", dir), "src/data\nsrc/util\nsrc/main");
        let _ = write(format!("{}/main.txt", dir), "src/main");
        let _ = write(format!("{}/single.txt", dir), ".");
    }
}

pub fn read_style(style: &str) -> Result<Vec<String>, String> {
    let dir = format!("{}/dzp_styles/{}.txt", get_dir(), style);

    match read_to_string(dir) {
        Ok(txt) => {
            Ok(txt
                .split_terminator('\n')
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
            )
        },
        Err(e) => Err(e.to_string())
    }
}