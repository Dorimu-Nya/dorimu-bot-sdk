use crate::{CommandDef, CommandHandleFn};
use inventory;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// 全局commands容器
pub static COMMANDS: Lazy<HashMap<&'static str, CommandHandleFn>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    inventory::iter::<CommandDef>.into_iter().for_each(|x| {
        map.insert(x.prefix, x.handler);
    });

    map
});
