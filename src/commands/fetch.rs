use crate::{discord_generic, read_language};
use serenity::{all::{CommandInteraction, Context}, builder::CreateCommand};
use std::process::Command;
use strip_ansi_escapes;

fn strip_ansi_codes(stringin: &str) -> String {
    let bytes = strip_ansi_escapes::strip(stringin);
    String::from_utf8(bytes).unwrap_or_default()
}

fn remove_spaces(stringin: String) -> String {
    let mut fixedstring: String = "".to_string();
    let bylines = stringin.split("\n");

    for line in bylines {
        fixedstring += line.trim_end();
        fixedstring += "\n";
    }

    return fixedstring;
}

pub async fn run(command : &CommandInteraction, ctx : &Context){
    let generic_error = read_language::get_message_string("error_generic".to_string());

    let ascii_image : String;
    let main_body : String;

    let image_output = Command::new("fastfetch")
    .arg("-s")
    .arg("logo")
    .output()
    .expect("failed to get ascii art");

    if image_output.status.success() {
        let unprocessed_ascii = String::from_utf8_lossy(&image_output.stdout).to_string();
        let processed_ascii = remove_spaces(strip_ansi_codes(unprocessed_ascii.as_str()));

        ascii_image = format!("```{}```", processed_ascii).to_string()
    } else {
        let stderr = str::from_utf8(&image_output.stderr).expect("invalid UTF-8");
        eprintln!("Error: {}",stderr);
        discord_generic::make_command_response(command, ctx, Some(generic_error), None).await;
        return;
    }

    let main_output = Command::new("fastfetch")
    .arg("--logo")
    .arg("none")
    .output()
    .expect("failed to get fetch info.");

    if main_output.status.success() {
        let unprocessed_body = String::from_utf8_lossy(&main_output.stdout).to_string();
        let processed_body = remove_spaces(strip_ansi_codes(unprocessed_body.as_str()));

        main_body = processed_body;
    } else {
        let stderr = str::from_utf8(&main_output.stderr).expect("invalid UTF-8");
        eprintln!("Error: {}",stderr);
        discord_generic::make_command_response(command, ctx, Some(generic_error), None).await;
        return;
    }

    discord_generic::make_command_response(command, ctx, Some(ascii_image), None).await;
    if let Err(why) = command.channel_id.say(&ctx.http, main_body).await {
        println!("Can't send message: {}", why)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("fetch").description("Gives you information about the developers computer.")
}
