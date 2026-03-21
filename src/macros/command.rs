
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct CommandDef {
    pub prefix: &'static str,
    // TODO 支持带参数的函数
    pub handler: fn(),
}

inventory::collect!(CommandDef);