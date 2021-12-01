use std::collections::BTreeMap;

use lazer::lazer;
use serde::{Serialize, Deserialize};
use serde_yaml::from_value;

use crate::cache::scripts::ScriptContents;

#[derive(Serialize, Deserialize, Clone)]
pub struct DocArg {
    #[serde(alias = "desc")]
    #[serde(alias = "about")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub arg_type: Option<String>,
}

impl DocArg {
    pub fn display(arg: Option<Self>, name: Option<String>, sep: &str) -> Option<String> {
        if let Some(arg) = arg {
            let t = match arg.arg_type {
                Some(t) => format!("[{}]", t),
                None => String::new()
            };
    
            let desc = match arg.description {
                Some(d) => format!("{} {}", sep, d),
                None => String::new()
            };
    
            let n = name.clone().unwrap_or_default() + if name.is_some() { " " } else { "" };
            Some(n + &t + &desc)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Doc {
    pub deprecated: Option<bool>,
    #[serde(alias = "desc")]
    #[serde(alias = "about")]
    pub description: Option<String>,
    pub usage: Option<String>,
    #[serde(alias = "determines")]
    pub determine: Option<DocArg>,
    #[serde(alias = "use")]
    pub uses: Option<Vec<String>>,
    #[serde(alias = "definitions")]
    pub defs: Option<BTreeMap<String, DocArg>>,
    pub keys: Option<BTreeMap<String, DocArg>>
}

impl Doc {
    pub fn from(contents: &ScriptContents) -> Option<Self> {
        let data = match contents.get("data") {
            Some(d) => d,
            None => contents.get("doc")?
        };
        
        from_value::<Self>(data.clone()).ok()
    }

    fn print_optional_str(key: &str, value: Option<String>) {
        lazer()
            .iff(value.is_some())
                .print_ln(&format!("{}{}", key, value.unwrap_or(String::new())))
            .end();
    }

    fn print_arg_list(key: &str, value: &Option<BTreeMap<String, DocArg>>) {
        let result = value.clone().unwrap_or_default().iter()
            .filter_map(|(name, arg)| DocArg::display(Some(arg.clone()), Some(name.clone()), ":"))
            .map(|s| format!("- {}", s))
            .collect::<Vec<String>>();

        lazer()
            .iff(value.is_some())
                .print_ln(&format!("\n{}:\n{}", key, result.join("\n")));
    }

    pub fn print(&self) {
        lazer()
            .iff(self.deprecated.unwrap_or(false))
                .print_yellow_ln("WARNING: This script is deprecated!\n");

        Self::print_optional_str("Description: ", self.description.clone());

        lazer()
            .iff(self.determine.is_some())
                .print_ln(&format!("Determine: {}", DocArg::display(self.determine.clone(), None, " -").unwrap_or_default()))
            .end()
            .iff(self.uses.is_some())
                .print_ln(&format!("Uses: {}", self.uses.clone().unwrap_or(Vec::new()).join(", ")))
            .end();

        Self::print_optional_str("\nUsage:\n", self.usage.clone());
        Self::print_arg_list("Defs", &self.defs);
        Self::print_arg_list("Data Keys:", &self.keys)
    }
}