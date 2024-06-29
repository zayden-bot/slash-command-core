use serenity::all::{Context, CreateCommand, CommandInteraction};
use async_trait::async_trait;

#[async_trait]
pub trait SlashCommand<E: std::error::Error> {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), E>;

    fn register() -> CreateCommand;
}