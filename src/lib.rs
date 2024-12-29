use async_trait::async_trait;
use serenity::all::{
    ActionRow, ActionRowComponent, CommandInteraction, Context, CreateCommand, Ready,
    ResolvedOption, ResolvedValue,
};
use std::collections::HashMap;

#[async_trait]
pub trait SlashCommand<E: std::error::Error> {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), E>;

    fn register(ctx: &Context, ready: &Ready) -> Result<CreateCommand, E>;
}

#[async_trait]
pub trait Autocomplete<E: std::error::Error> {
    async fn autocomplete(ctx: &Context, interaction: &CommandInteraction) -> Result<(), E>;
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

fn get_option_str(options: &[ResolvedOption<'_>]) -> String {
    let mut s = String::new();

    for option in options {
        s.push(' ');
        s.push_str(option.name);

        match &option.value {
            ResolvedValue::SubCommandGroup(sub_options) => {
                s.push_str(&get_option_str(sub_options));
            }
            ResolvedValue::SubCommand(sub_options) => {
                for sub_option in sub_options {
                    s.push(' ');
                    s.push_str(sub_option.name);
                    s.push_str(": ");
                    s.push_str(&format!("{:?}", sub_option.value));
                }
            }
            _ => {}
        }
    }

    s
}

pub fn parse_modal_data(components: &[ActionRow]) -> HashMap<&str, &str> {
    components
        .iter()
        .flat_map(|action_row| action_row.components.iter())
        .filter_map(|component| {
            if let ActionRowComponent::InputText(input) = component {
                input
                    .value
                    .as_deref()
                    .map(|value| (input.custom_id.as_str(), value))
            } else {
                None
            }
        })
        .collect()
}
