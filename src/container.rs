use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::CommandDef;

pub static COMMANDS: Lazy<HashMap<&'static str, fn()>> = Lazy::new(|| {
    let mut map = HashMap::new();


    for x in inventory::iter::<CommandDef> {
        map.insert(x.prefix, x.handler);
    }

    map
});