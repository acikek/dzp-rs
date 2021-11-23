use clap::{App, load_yaml};

mod config;
mod io;
mod structs;
mod subcmd;

fn main() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support();

    let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);

    config::styles::init_styles(true);

    let _ = match app.clone().get_matches().subcommand() {
        ("new", Some(sub)) => subcmd::new::new(&sub),
        ("project", Some(sub)) => subcmd::project::project(&sub),
        ("script", Some(sub)) => subcmd::script::script(&sub),
        _ => { let _ = app.print_help(); }
    };
}
