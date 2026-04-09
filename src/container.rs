use crate::{CommandDef, CommandHandleFn};
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// 全局commands容器
pub static COMMANDS: Lazy<HashMap<&'static str, CommandHandleFn>> = Lazy::new(|| {
    let mut map = HashMap::new();

    for x in inventory::iter::<CommandDef> {
        map.insert(x.prefix, x.handler);
    }

    map
});
