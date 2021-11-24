use lazer::lazer;
use license::from_id;
use rustyline::Editor;
use semver::Version;
use serde::{Serialize, Deserialize};
use serde_yaml::to_string;
use url::Url;

use crate::io::log::err;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectData {
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub version: Version,
    pub license: String,
    pub homepage: Option<Url>,
    pub repository: Option<Url>,
    pub dependencies: Option<Vec<String>>
}

type Error = rustyline::error::ReadlineError;

impl ProjectData {
    fn input(text: &str, rl: &mut Editor<()>) -> Result<String, Error> {
        let line = rl.readline(&format!("{}: ", text))?;
        rl.add_history_entry(&line);
        Ok(line.trim().to_string())
    }

    fn input_authors(rl: &mut Editor<()>) -> Result<Vec<String>, Error> {
        let input = Self::input("Authors", rl)?;

        Ok(input
            .split_terminator(',')
            .map(|s| String::from(s.trim()))
            .filter(|s| !s.is_empty())
            .collect())
    }

    fn input_version(rl: &mut Editor<()>) -> Result<Version, Error> {
        loop {
            let input = Self::input("Version", rl)?;
            if input.is_empty() { return Ok(Version::new(0, 1, 0)) }

            let v = Version::parse(&input);
    
            match v {
                Ok(version) => return Ok(version),
                Err(_) => err("Invalid version.", None)
            }
        }
    }
    
    fn input_url(text: &str, rl: &mut Editor<()>) -> Result<Option<Url>, Error> {
        loop {
            let input = Self::input(text, rl)?;
            if input.is_empty() { return Ok(None) }

            let u = Url::parse(&input);
    
            match u {
                Ok(url) => return Ok(Some(url)),
                Err(_) => err("Invalid URL.", None)
            }
        }
    }

    fn input_license(rl: &mut Editor<()>) -> Result<String, Error> {
        loop {
            let input = Self::input("License", rl)?;
            if input.is_empty() { return Ok(String::from("MIT")) }

            let l = from_id(&input);

            match l {
                Some(_) => return Ok(String::from(input)),
                None => err("Invalid license.", None)
            }
        }
    }

    pub fn from_input(name: String) -> Result<Self, Error> {
        let mut rl = Editor::<()>::new();

        let description: String = Self::input("Description", &mut rl)?;
        let authors = Self::input_authors(&mut rl)?;
        let version = Self::input_version(&mut rl)?;
        let license = Self::input_license(&mut rl)?;
        let homepage = Self::input_url("Homepage", &mut rl)?;
        let repository = Self::input_url("Repository", &mut rl)?;

        println!();

        Ok(Self {
            name,
            description,
            authors,
            version,
            license,
            homepage,
            repository,
            dependencies: Some(Vec::new())
        })
    }

    pub fn from_name(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            authors: Vec::<String>::new(),
            version: Version::new(0, 1, 0),
            license: String::from("MIT"),
            homepage: None,
            repository: None,
            dependencies: Some(Vec::new())
        }
    }

    pub fn readme_header(&self) -> String {
        format!("# {}\n\n{}", self.name,  if self.description.is_empty() {
            String::from("Sample description")
        } else {
            self.description.clone()
        })
    }

    pub fn license_text(&self) -> String {
        from_id(&self.license).unwrap_or(&license::MIT).text().to_string()
    }

    pub fn yaml(&self) -> String {
        to_string(&self).ok().unwrap()
    }

    pub fn print_url(key: &str, url: Option<Url>) {
        match url {
            Some(u) => println!("{}: {}", key, u),
            None => ()
        }
    }

    pub fn print(&self) {
        lazer()
            .print(&self.name)
            .print_ln(&format!(" v{}", self.version))
            .iff(self.description.is_empty())
                .print_ln("No description")
            .el()
                .print_ln(&self.description)
            .end()
            .print_ln("")
            .iff(self.authors.len() > 1)
                .print_ln(&format!("Authors: {}", self.authors.join(", ")))
            .el()
                .print_ln(&format!("Author: {}", self.authors.first().unwrap_or(&"None".to_owned())))
            .end()
            .print_ln(&format!("License: {}", self.license));

        Self::print_url("Homepage", self.homepage.clone());
        Self::print_url("Repository", self.repository.clone());

        lazer()
            .iff(self.dependencies.is_some() && !self.dependencies.clone().unwrap().is_empty())
                .print_ln("\nDependencies:")
                .print_ln(&self.dependencies.clone().unwrap_or(Vec::new()).iter().map(|s| format!("- {}", s)).collect::<Vec<String>>().join("\n"))
            .end();
    }
}