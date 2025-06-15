use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use serenity::all::MessageId;
use serenity::prelude::TypeMapKey;

#[derive(Default)]
pub struct ActiveMessages(HashMap<String, MessageId>);

impl ActiveMessages {
    pub fn insert(&mut self, cmd: impl Into<String>, id: impl Into<MessageId>) {
        self.0.insert(cmd.into(), id.into());
    }

    pub fn check(&self, cmd: &str) -> bool {
        self.0.contains_key(cmd)
    }
}

impl Deref for ActiveMessages {
    type Target = HashMap<String, MessageId>;

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
