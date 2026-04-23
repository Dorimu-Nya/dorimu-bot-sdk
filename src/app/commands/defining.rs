use super::super::context::ContextStore;
use super::replying::ReplyingMessage;
use crate::events::common::CommonMessage;
use std::{fmt::Display, future::Future, pin::Pin};

// 错误的封装
// sheip9 (2026/4/9): 不知道标准库里有没有啥东西可以替换他
pub type BoxDisplay = Box<dyn Display + Send + Sync>;

/// command 函数的类型声明
pub type CommandHandleFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Option<ReplyingMessage>, BoxDisplay>> + Send + 'a>>;

/// 命令处理函数类型
///
/// 接收消息和依赖容器，返回异步的命令处理结果
pub type CommandHandleFn =
    for<'a> fn(&'a dyn CommonMessage, &'a ContextStore) -> CommandHandleFuture<'a>;

/// command 函数的返回值的统一转换trait
pub trait CommandOutput {
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay>;
}

/// 将无返回值 `()` 转换为 `CommandOutput`。
///
/// 对应的 `into_output` 返回 `Ok(None)`，表示不需要回复消息。
impl CommandOutput for () {
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        Ok(None)
    }
}

/// 将 `ReplyingMessage` 本身作为 `CommandOutput`。
///
/// 对应的 `into_output` 将消息包装为 `Some(reply)` 并返回 `Ok(Some(reply))`。
impl CommandOutput for ReplyingMessage {
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        Ok(Some(self))
    }
}

/// 将带错误类型的 `Result<ReplyingMessage, E>` 转换为统一输出形式。
///
/// - 成功时（`Ok(reply)`）包装为 `Some(reply)` 并返回 `Ok(Some(reply))`。
/// - 失败时（`Err(e)`）将错误封装为 `BoxDisplay`（即 `Box<dyn Display + Send + Sync>`）并作为 `Err` 返回。
impl<E> CommandOutput for Result<ReplyingMessage, E>
where
    E: Display + Send + Sync + 'static,
{
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        self.map(Some).map_err(|err| Box::new(err) as BoxDisplay)
    }
}

/// 将带错误类型的 `Result<Option<ReplyingMessage>, E>` 转换为统一输出形式。
///
/// - 成功时直接返回内部的 `Option<ReplyingMessage>`，保持 `Some`/`None` 语义不变。
/// - 失败时将错误转换为 `BoxDisplay` 并作为 `Err` 返回。
impl<E> CommandOutput for Result<Option<ReplyingMessage>, E>
where
    E: Display + Send + Sync + 'static,
{
    fn into_output(self) -> Result<Option<ReplyingMessage>, BoxDisplay> {
        self.map_err(|err| Box::new(err) as BoxDisplay)
    }
}

/// command 宏的储存定义
#[derive(Debug)]
pub struct CommandDef {
    pub prefix: &'static str,
    pub handler: CommandHandleFn,
}

#[cfg(feature = "macros")]
inventory::collect!(CommandDef);
