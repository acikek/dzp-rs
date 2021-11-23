use ansi_term::Color::{Red, Black};

pub fn err(text: &str, reason: Option<String>) {
    let msg = Red.paint(text);

    match reason {
        Some(r) => {
            let rsn = Black.bold().paint(r);
            println!("{} {}", msg, rsn);
        }
        None => println!("{}", msg)
    }
}