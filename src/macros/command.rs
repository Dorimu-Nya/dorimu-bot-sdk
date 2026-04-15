use crate::context::ContextStore;
use crate::{events::common::CommonMessage, replying::ReplyingMessage};
use std::{fmt::Display, future::Future, pin::Pin};

// 错误的封装
// sheip9 (2026/4/9): 不知道标准库里有没有啥东西可以替换他
pub type BoxDisplay = Box<dyn Display + Send + Sync>;

/// command函数的类型声明
pub type CommandHandleFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Option<ReplyingMessage>, BoxDisplay>> + Send + 'a>>;

/// 命令处理函数类型
///
/// 接收消息和依赖容器，返回异步的命令处理结果
pub type CommandHandleFn =
    for<'a> fn(&'a dyn CommonMessage, &'a ContextStore) -> CommandHandleFuture<'a>;

/// 输出的统一转换trait
pub trait CommandOutput {
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay>;
}

// 无返回的转换
impl CommandOutput for () {
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        Ok(None)
    }
}

// ReplyingMessage 对 command方法返回的转换
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
        self.map(Some).map_err(|err| Box::new(err) as BoxDisplay)
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

/// command宏的储存定义
#[derive(Debug)]
pub struct CommandDef {
    pub prefix: &'static str,
    pub handler: CommandHandleFn,
}

inventory::collect!(CommandDef);
