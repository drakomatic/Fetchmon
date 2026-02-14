use chrono::{Duration, Utc};
use serenity::{
    all::{ChannelType, CommandInteraction, Context, Timestamp},
    builder::CreateCommand,
};

use crate::{discord_generic, read_language};

pub async fn run(command: &CommandInteraction, ctx: &Context) {
    if let Some(guild_id) = command.guild_id {
        let channels = match guild_id.channels(&ctx.http).await {
            Ok(channels) => channels,
            Err(why) => {
                println!("Unable to get channels: {}", why);
                return;
            }
        };

        let timeout_time = Timestamp::from(Utc::now() + Duration::hours(1));

        for (_, channel) in channels {
            if channel.kind != ChannelType::Voice {
                continue;
            }

            let members = match channel.members(&ctx.cache) {
                Ok(members) => members,
                Err(why) => {
                    println!("Unable to get members: {}", why);
                    return;
                }
            };

            for mut member in members {
                if !discord_generic::can_moderate_user(ctx, guild_id, &member){
                    continue;
                }

                if let Err(why) = member.disable_communication_until_datetime(&ctx.http, timeout_time).await {
                    println!("Cannot timeout user: {}", why)
                }
            }
        }
    } else {
        let response_text =
            read_language::get_message_string("msg_error_out_of_server".to_string());
        discord_generic::make_command_response(command, ctx, Some(response_text), None).await;
        return;
    }

    let response_text = read_language::get_message_string("msgset_reply_sleep".to_string());
    discord_generic::make_command_response(
        command,
        ctx,
        Some(response_text),
        Some("content/images/renanight.gif"),
    )
    .await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("sleep").description("Times out everyone in voice chat.")
}
