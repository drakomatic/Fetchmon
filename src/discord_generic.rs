use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateInteractionResponse,
    CreateInteractionResponseMessage, GuildId, Member,
};

pub async fn make_command_response(
    command: &CommandInteraction,
    ctx: &Context,
    response: Option<String>,
    file_attachment: Option<&str>,
) {
    let mut data = CreateInteractionResponseMessage::new();

    if let Some(response) = response {
        data = data.content(response);
    }

    if let Some(file_attachment) = file_attachment {
        let image_file: CreateAttachment;

        match CreateAttachment::path(file_attachment).await {
            Ok(created) => image_file = created,
            Err(why) => {
                eprintln!("An error occured: {}", why);
                return;
            }
        }

        data = data.add_file(image_file);
    }

    let builder = CreateInteractionResponse::Message(data);

    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub fn can_moderate_user(ctx: &Context, guild_id: GuildId, target: &Member) -> bool {
    // Get guild from cache
    let Some(guild) = ctx.cache.guild(guild_id) else {
        return false;
    };

    let bot_id = ctx.cache.current_user().id;

    // Cannot moderate server owner
    if target.user.id == guild.owner_id {
        return false;
    }

    // Get bot as guild member
    let Some(bot_member) = guild.members.get(&bot_id) else {
        return false;
    };

    // Get highest roles (NEW API)
    let bot_role = guild.member_highest_role(bot_member);
    let target_role = guild.member_highest_role(target);

    // Compare role positions safely
    match (bot_role, target_role) {
        (Some(bot_role), Some(target_role)) => {
            bot_role.position > target_role.position
        }
        (Some(_), None) => true,  // target has no roles
        _ => false,
    }
}
