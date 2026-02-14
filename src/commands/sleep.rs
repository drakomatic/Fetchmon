use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("sleep").description("Times out everyone in voice chat.")
}