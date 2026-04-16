use crate::app::commands::defining::{CommandDef, CommandHandleFn};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct CommandsStore {
    pub commands: Arc<HashMap<&'static str, CommandHandleFn>>,
}

impl CommandsStore {
    pub fn new(commands: HashMap<&'static str, CommandHandleFn>) -> CommandsStore {
        CommandsStore {
            commands: Arc::new(commands),
        }
    }

    pub fn get(&self, prefix: &str) -> Option<CommandHandleFn> {
        self.commands.get(prefix).cloned()
    }
}
