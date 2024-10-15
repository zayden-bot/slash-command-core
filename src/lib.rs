use async_trait::async_trait;
use serenity::all::{
    ActionRow, ActionRowComponent, CommandInteraction, Context, CreateCommand, InputText,
    ResolvedOption, ResolvedValue,
};
use std::collections::HashMap;

#[async_trait]
pub trait SlashCommand<E: std::error::Error> {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), E>;

    fn register() -> CreateCommand;
}

pub trait ErrorResponse {
    fn to_response(&self) -> String;
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

pub fn parse_modal_data(components: &[ActionRow]) -> HashMap<&str, &InputText> {
    components
        .iter()
        .flat_map(|action_row| action_row.components.iter())
        .filter_map(|component| {
            if let ActionRowComponent::InputText(input) = component {
                Some((input.custom_id.as_str(), input))
            } else {
                None
            }
        })
        .collect()
}
