use std::collections::HashMap;
use std::sync::Arc;
use qqbot_sdk::commands::defining::{CommandDef, CommandHandleFn};

#[derive(Clone)]
pub struct CommandsStore {
    pub commands: Arc<HashMap<&'static str, CommandHandleFn>>,
}

impl CommandsStore {
    pub fn new() -> CommandsStore {
        let mut commands = HashMap::new();

        #[cfg(feature = "macros")]
        inventory::iter::<CommandDef>.into_iter().for_each(|x| {
            commands.insert(x.prefix, x.handler);
        });

        CommandsStore { commands: Arc::new(commands) }
    }

    pub fn get(&self, prefix: &str) -> Option<CommandHandleFn> {
        self.commands.get(prefix).cloned()
    }
}