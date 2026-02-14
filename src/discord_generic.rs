use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateInteractionResponse,
    CreateInteractionResponseMessage,
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
