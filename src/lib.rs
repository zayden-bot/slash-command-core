use std::collections::HashMap;
use serenity::all::{Context, CreateCommand, CommandInteraction, ResolvedOption, ResolvedValue};
use async_trait::async_trait;

#[async_trait]
pub trait SlashCommand<E: std::error::Error> {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), E>;

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
