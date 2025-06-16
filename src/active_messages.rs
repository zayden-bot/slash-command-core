use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use serenity::all::UserId;
use serenity::prelude::TypeMapKey;

use crate::Error;

#[derive(Default)]
pub struct ActiveMessages(HashSet<(String, UserId)>);

impl ActiveMessages {
    pub fn insert(&mut self, cmd: impl Into<String>, id: impl Into<UserId>) -> bool {
        self.0.insert((cmd.into(), id.into()))
    }

    pub fn check(&self, cmd: impl Into<String>, id: impl Into<UserId>) -> Result<(), Error> {
        if self.0.contains(&(cmd.into(), id.into())) {
            Err(Error::MessageConflict)
        } else {
            Ok(())
        }
    }

    pub fn remove(&mut self, cmd: impl Into<String>, id: impl Into<UserId>) -> bool {
        self.0.remove(&(cmd.into(), id.into()))
    }
}

impl Deref for ActiveMessages {
    type Target = HashSet<(String, UserId)>;

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
