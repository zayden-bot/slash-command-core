use serenity::all::{Context, CreateCommand, CommandInteraction};

#[async_trait]
pub trait SlashCommand<E: std::error::Error> {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), E>;

    fn register() -> CreateCommand;
}