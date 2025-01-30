use std::collections::HashMap;

use async_trait::async_trait;
use serenity::all::{
    ActionRow, ActionRowComponent, AutocompleteOption, CommandInteraction, ComponentInteraction,
    Context, CreateCommand, Message, ModalInteraction, Ready, ResolvedOption, ResolvedValue,
};
use sqlx::{Database, Pool};

#[async_trait]
pub trait SlashCommand<E: std::error::Error> {
    async fn run<Db: Database>(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &Pool<Db>,
    ) -> Result<(), E>;

    fn register(ctx: &Context, ready: &Ready) -> Result<CreateCommand, E>;
}

#[async_trait]
pub trait Autocomplete<E: std::error::Error> {
    async fn autocomplete(
        ctx: &Context,
        interaction: &CommandInteraction,
        option: AutocompleteOption<'_>,
    ) -> Result<(), E>;
}

#[async_trait]
pub trait Component<E: std::error::Error> {
    async fn run<Db: Database>(
        ctx: &Context,
        interaction: &ComponentInteraction,
        pool: &Pool<Db>,
    ) -> Result<(), E>;
}

#[async_trait]
pub trait Modal<E: std::error::Error> {
    async fn run<Db: Database>(
        ctx: &Context,
        interaction: &ModalInteraction,
        components: &[ActionRow],
        pool: &Pool<Db>,
    ) -> Result<(), E>;
}

#[async_trait]
pub trait MessageCommand<E: std::error::Error> {
    async fn run<Db: Database>(ctx: &Context, message: &Message, pool: &Pool<Db>) -> Result<(), E>;
}

pub trait ErrorResponse {
    fn to_response(&self) -> &str;
}

pub enum Error {
    MissingGuildId,
    NotInteractionAuthor,
}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Error::MissingGuildId => "This command can only be used within a server.",
            Error::NotInteractionAuthor => "You are not the author of this interaction.",
        }
    }
}

pub fn parse_options<'a>(
    options: impl IntoIterator<Item = ResolvedOption<'a>>,
) -> HashMap<&'a str, ResolvedValue<'a>> {
    options
        .into_iter()
        .map(|option| (option.name, option.value))
        .collect()
}

pub fn get_option_str(options: &[ResolvedOption<'_>]) -> String {
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
        .flat_map(|action_row| &action_row.components)
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
