use crate::{events::common::CommonMessage, replying::ReplyingMessage};
use std::{fmt::Display, future::Future, pin::Pin};

pub type BoxDisplay = Box<dyn Display + Send + Sync>;

pub type CommandHandleFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Option<ReplyingMessage>, BoxDisplay>> + Send + 'a>>;

pub type CommandHandleFn = for<'a> fn(&'a dyn CommonMessage) -> CommandHandleFuture<'a>;

pub trait CommandOutput {
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay>;
}

impl CommandOutput for () {
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        Ok(None)
    }
}

impl CommandOutput for ReplyingMessage {
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        Ok(Some(self))
    }
}

impl<E> CommandOutput for Result<ReplyingMessage, E>
where
    E: Display + Send + Sync + 'static,
{
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        self.map(Some)
            .map_err(|err| Box::new(err) as BoxDisplay)
    }
}

impl<E> CommandOutput for Result<Option<ReplyingMessage>, E>
where
    E: Display + Send + Sync + 'static,
{
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        self.map_err(|err| Box::new(err) as BoxDisplay)
    }
}

#[derive(Debug)]
pub struct CommandDef {
    pub prefix: &'static str,
    pub handler: CommandHandleFn,
}

inventory::collect!(CommandDef);
