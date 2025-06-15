use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use serenity::all::{Context, Guild, GuildId, MessageId, UserId};
use serenity::prelude::TypeMapKey;

pub struct ActiveMessages(HashSet<MessageId>);

impl ActiveMessages {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}

impl Deref for ActiveMessages {
    type Target = HashSet<MessageId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ActiveMessages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeMapKey for ActiveMessages {
    type Value = ActiveMessages;
}
