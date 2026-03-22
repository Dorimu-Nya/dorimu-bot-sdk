use crate::events::common::CommonMessage;

pub type CommandHandleFn = fn(&dyn CommonMessage);

#[derive(Debug)]
pub struct CommandDef {
    pub prefix: &'static str,
    pub handler: CommandHandleFn,
}

inventory::collect!(CommandDef);
