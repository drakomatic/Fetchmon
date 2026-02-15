use serenity::all::{ChannelType, CommandInteraction, Context, CreateCommand, PremiumTier};

use crate::{discord_generic, read_language};

pub async fn run(command: &CommandInteraction, ctx: &Context) {
    if command.guild_id.is_none() {
        let errormessage = read_language::get_message_string("msg_error_out_of_server".to_string());

        discord_generic::make_command_response(command, ctx, Some(errormessage), None).await;
        return;
    }
    // info gathering
    let guild_id = command.guild_id.expect("Unable to recieve guild id");

    let guild_name = guild_id.name(&ctx.cache).expect("Guild name missing");
    let guild_id_string = guild_id.get();

    let guild_description = {
        let guild_cache = guild_id.to_guild_cached(&ctx.cache).unwrap();

        guild_cache
            .description
            .clone()
            .unwrap_or_else(|| "No description".to_string())
    };

    let creation_date = guild_id.created_at();
    let member_count = {
        let guild_cache = guild_id.to_guild_cached(&ctx.cache).unwrap();
        guild_cache.member_count
    };
    let max_member_count = {
        let guild_cache = guild_id.to_guild_cached(&ctx.cache).unwrap();
        guild_cache.max_members.unwrap_or_else(|| 0)
    };

    let channel_count = {
        let guild_cache = guild_id.to_guild_cached(&ctx.cache).unwrap();
        guild_cache.channels.len()
    };
    let voice_channel_count = guild_id
        .channels(&ctx.http)
        .await
        .expect("Unable to get channels")
        .values()
        .filter(|channel| channel.kind == ChannelType::Voice)
        .count();
    let max_bit_rate = {
        let guild_cache = guild_id.to_guild_cached(&ctx.cache).unwrap();
        match guild_cache.premium_tier {
            PremiumTier::Tier0 => 96,
            PremiumTier::Tier1 => 128,
            PremiumTier::Tier2 => 256,
            PremiumTier::Tier3 => 384,
            _ => 96,
        }
    };

    // response building
    let mut output_string: String = "".to_string();

    output_string += format!(
        "**{}@discord.com | GuildId: {}**",
        guild_name.as_str(),
        guild_id_string
    )
    .as_str();
    output_string += "\n-- ==== [ Description ] ==== --\n";
    output_string += (guild_description + "\n").as_str();
    output_string += "-- ==== [ Member Info ] ==== --\n";
    output_string += format!(
        "Creation date: {}\nMember count: {}/{}\n",
        creation_date.date_naive().format("%m/%d/%Y"),
        member_count,
        max_member_count
    )
    .as_str();
    output_string += "-- ==== [ Channel Info ] ==== --\n";
    output_string += format!(
        "Channel count: {}\nVoice channel count: {}\nMaximum bitrate: {}kbps\n",
        channel_count, voice_channel_count, max_bit_rate
    )
    .as_str();

    discord_generic::make_command_response(command, ctx, Some(output_string), None).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("serverinfo")
        .description("Gives information about the current server, fetch style.")
}
