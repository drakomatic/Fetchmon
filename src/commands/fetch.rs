use serenity::{all::ResolvedOption, builder::CreateCommand};
use std::process::Command;
use strip_ansi_escapes;
use crate::read_language;

fn strip_ansi_codes(stringin : &str) -> String {
    let bytes = strip_ansi_escapes::strip(stringin);
    String::from_utf8(bytes).unwrap_or_default()
}

fn remove_spaces(stringin : String) -> String {
    let mut fixedstring : String = "".to_string();
    let bylines = stringin.split("\n");

    for line in bylines {
        fixedstring += line.trim_end();
        fixedstring += "\n";
    }

    return fixedstring;
}

pub fn run(_options: &[ResolvedOption]) -> String {
    //is this how you do arbitrary code?
    let output = Command::new("fastfetch")
        .arg("-s")
        .arg("logo")
        .output()
        .expect("failed to execute fetch.");



    if output.status.success() {
        let textart: String = String::from_utf8_lossy(&output.stdout).to_string();
        let mut textart_clean = strip_ansi_codes(textart.as_str());
        textart_clean = remove_spaces(textart_clean);

        return format!("```{}```", textart_clean).to_string();
    } else {
        let stderr = str::from_utf8(&output.stderr).expect("Not valid UTF-8");
        eprintln!("Error: {}", stderr);
        return read_language::get_message_string("error_generic".to_string());
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("fetch").description("Gives you information about the developers computer.")
}