use crate::app::commands::defining::DynCommandHandleFn;
use std::collections::HashMap;
use std::sync::Arc;

/// command 函数 的存储
#[derive(Clone)]
pub struct CommandsStore {
    pub commands: Arc<HashMap<&'static str, DynCommandHandleFn>>,
}

impl CommandsStore {
    pub fn new(commands: HashMap<&'static str, DynCommandHandleFn>) -> CommandsStore {
        CommandsStore {
            commands: Arc::new(commands),
        }
    }

    pub fn get(&self, prefix: &str) -> Option<DynCommandHandleFn> {
        self.commands.get(prefix).map(Arc::clone)
    }
}
