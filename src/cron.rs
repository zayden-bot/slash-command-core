use std::{error::Error, pin::Pin, str::FromStr};

use cron::Schedule;
use serenity::all::Context;
use sqlx::{Database, Pool};

pub type ActionFn<Db, E> = Box<
    dyn Fn(Context, Pool<Db>) -> Pin<Box<dyn Future<Output = Result<(), E>> + Send>> + Send + Sync,
>;

pub struct CronJob<Db: Database, E: Error> {
    pub schedule: Schedule,
    action_fn: ActionFn<Db, E>,
}

impl<Db, E> CronJob<Db, E>
where
    Db: Database,
    E: Error,
{
    pub fn new(source: &str) -> Self {
        Self {
            schedule: Schedule::from_str(source).unwrap(),
            action_fn: Self::action_fn(|_, _| async { Ok(()) }),
        }
    }

    fn action_fn<F, Fut>(f: F) -> ActionFn<Db, E>
    where
        F: Fn(Context, Pool<Db>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), E>> + Send + 'static,
    {
        let action_closure = move |ctx, pool| {
            let future = f(ctx, pool);
            let boxed_dyn_future: Box<dyn Future<Output = Result<(), E>> + Send> = Box::new(future);

            let pinned_future: Pin<Box<dyn Future<Output = Result<(), E>> + Send>> =
                Box::into_pin(boxed_dyn_future);

            pinned_future
        };

        Box::new(action_closure)
    }

    pub fn set_action<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Context, Pool<Db>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), E>> + Send + 'static,
    {
        self.action_fn = Self::action_fn(f);
        self
    }
}
