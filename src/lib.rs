use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, ResolvedValue};
use sqlx::PgPool;
use std::collections::HashMap;

#[async_trait]
pub trait SlashCommand<E: std::error::Error> {
    async fn run(ctx: &Context, interaction: &CommandInteraction, pool: &PgPool) -> Result<(), E>;

    fn register() -> CreateCommand;
}

pub fn parse_options<'a>(
    options: &'a Vec<ResolvedOption<'_>>,
) -> HashMap<&'a str, &'a ResolvedValue<'a>> {
    let mut parsed_options = HashMap::with_capacity(options.len());

    for option in options {
        parsed_options.insert(option.name, &option.value);
    }

    parsed_options
}
