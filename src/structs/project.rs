use lazer::lazer;
use license::from_id;
use rustyline::Editor;
use semver::Version;
use serde::{Serialize, Deserialize};
use serde_yaml::to_string;
use titlecase::titlecase;
use url::Url;

use crate::cache::scripts::get_script_types;
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

    pub fn title(&self) -> String {
        format!("{} v{}", self.name, self.version)
    }

    pub fn description_default(&self) -> String {
        if self.description.is_empty() {
            String::from("No description")
        } else {
            self.description.clone()
        }
    }

    pub fn authors_default(&self) -> String {
        if self.authors.is_empty() {
            String::from("None")
        } else {
            self.authors.join(", ")
        }
    }

    pub fn readme_header(&self) -> String {
        format!("# {}\n\n{}", self.name, self.description_default())
    }

    const README_ABOUT: &'static str = "## About\n\nProvide a brief explanation of your project and how it's used.";
    const README_EXAMPLE: &'static str = "## Example\n\nUse this space to demonstrate the usage of your project.\n> If the user only needs to worry about installation, consider omitting this section.";
    const README_DZP: &'static str = "Use [dzp](https://github.com/acikek/dzp-rs) for additional features.";

    fn readme_setup(&self) -> String {
        let repo = match &self.repository {
            Some(u) => u.to_string(),
            None => "https://github.com/<user>/<name>".to_owned()
        };

        format!("## Setup\n\nClone using git:\n```sh\ngit clone {}\n```\n{}", repo, Self::README_DZP)
    }

    fn readme_scripts(&self) -> String {
        let scripts = get_script_types().iter()
            .map(|(script_type, names)| {
                let t = titlecase(script_type);
                let n = names.iter()
                    .map(|s| format!("- `{}`", s))
                    .collect::<Vec<String>>()
                    .join("\n");

                format!("### {}\n\n{}", t, n)
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        format!("## Scripts\n\n{}", scripts)
    }

    pub fn readme_license(&self) -> String {
        format!("## License\n\n{} © (year) (name)", self.license)
    }

    pub fn readme(&self) -> String {
        let sections = vec![
            self.readme_header(),
            Self::README_ABOUT.to_owned(),
            self.readme_setup(),
            Self::README_EXAMPLE.to_owned(),
            self.readme_scripts(),
            self.readme_license()
        ];

        sections.join("\n\n")
    }

    pub fn comment_key(key: &str, value: String) -> String {
        format!("#| @{} {}", key, value)
    }

    pub fn comment_header(&self) -> String {
        format!("#| {}\n#| {}\n#|\n{}\n{}",
            self.title(),
            self.description_default(),
            Self::comment_key("author", self.authors_default()),
            Self::comment_key("license", self.license.clone())
        )
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