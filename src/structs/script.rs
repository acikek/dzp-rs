use lazer::lazer;
use serde::{Serialize, Deserialize};
use serde_yaml::from_value;

use crate::cache::scripts::ScriptContents;

#[derive(Serialize, Deserialize)]
pub struct Script {
    pub path: String,
    #[serde(rename = "type")]
    pub script_type: String,
    pub keys: Vec<String>
}

impl Script {
    pub fn from((path, contents): (String, ScriptContents)) -> Result<Self, String> {
        if !contents.contains_key("type") {
            return Err("Script does not contain a 'type' key.".to_owned());
        }

        match from_value(contents.get("type").unwrap().clone()) {
            Ok(script_type) => {
                let keys = contents.keys().map(|s| s.clone()).collect();

                Ok(Self {
                    path,
                    script_type,
                    keys
                })
            },
            Err(_) => Err("Failed to parse 'type' key.".to_owned())
        }
    }

    pub fn print(&self, name: &str) {
        lazer()
            .print_ln(&format!("Name: {}", name))
            .print_ln(&format!("Type: {}", self.script_type))
            .print_ln(&format!("Path: {}", self.path))
            .print_ln(&format!("Keys: {}", self.keys.join(", ")));
    }
}