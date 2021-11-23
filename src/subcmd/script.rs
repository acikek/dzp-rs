use clap::ArgMatches;

use crate::io::log::err; 
use crate::cache::scripts::get_scripts;

pub fn script(matches: &ArgMatches) {
    let scripts = get_scripts();

    match matches.value_of("NAME") {
        Some(name) => {
            match scripts.get(name) {
                Some(s) => {
                    println!();
                    s.print(name);
                }
                None => err("Failed to find script.", None)
            }
        },
        None => {
            let keys = scripts.keys()
                .clone()
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
                .join(", ");

            println!("{}", keys);
        }
    }
}