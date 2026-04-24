use super::super::context::{Context, ContextStore};
use super::replying::ReplyingMessage;
use crate::events::common::{CommonMessage, FromCommonMessage};
use std::{fmt::Display, future::Future, pin::Pin, sync::Arc};

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

/// 动态命令处理函数类型（用于手动注册与统一存储）。
pub type DynCommandHandleFn = Arc<
    dyn for<'a> Fn(&'a dyn CommonMessage, &'a ContextStore) -> CommandHandleFuture<'a>
        + Send
        + Sync,
>;

/// 将函数指针类型的命令处理器包装为动态处理器。
pub fn wrap_command_handle_fn(handler: CommandHandleFn) -> DynCommandHandleFn {
    Arc::new(move |message, store| handler(message, store))
}

/// 手动注册命令时的参数提取 trait。
///
/// - 默认从 `FromCommonMessage` 提取。
/// - `Context<T>` 会从 `ContextStore` 中注入。
pub trait FromCommandArg: Sized {
    fn from_command_arg(message: &dyn CommonMessage, store: &ContextStore) -> Self;
}

impl<T> FromCommandArg for T
where
    for<'a> T: FromCommonMessage<'a>,
{
    fn from_command_arg(message: &dyn CommonMessage, _store: &ContextStore) -> Self {
        <Self as FromCommonMessage<'_>>::from(message)
    }
}

impl<T> FromCommandArg for Context<T>
where
    T: Send + Sync + 'static,
{
    fn from_command_arg(_message: &dyn CommonMessage, store: &ContextStore) -> Self {
        store.get_context::<T>()
    }
}

/// 同步命令函数的适配标记。
pub struct SyncCommandHandlerKind;
/// 异步命令函数的适配标记。
pub struct AsyncCommandHandlerKind;

/// 将普通函数适配为统一命令处理函数的 trait。
pub trait CommandHandler<Args, Kind>: Send + Sync + 'static {
    fn into_dyn(self) -> DynCommandHandleFn;
}

macro_rules! impl_command_handler {
    () => {
        impl<F, R> CommandHandler<(), SyncCommandHandlerKind> for F
        where
            F: Fn() -> R + Send + Sync + 'static,
            R: CommandOutput + Send + 'static,
        {
            fn into_dyn(self) -> DynCommandHandleFn {
                Arc::new(move |_message, _store| {
                    let result = (self)();
                    Box::pin(async move { CommandOutput::into_output(result) })
                })
            }
        }

        impl<F, Fut, R> CommandHandler<(), AsyncCommandHandlerKind> for F
        where
            F: Fn() -> Fut + Send + Sync + 'static,
            Fut: Future<Output = R> + Send + 'static,
            R: CommandOutput + Send + 'static,
        {
            fn into_dyn(self) -> DynCommandHandleFn {
                Arc::new(move |_message, _store| {
                    let fut = (self)();
                    Box::pin(async move {
                        let result = fut.await;
                        CommandOutput::into_output(result)
                    })
                })
            }
        }
    };
    ($( $ty:ident => $var:ident ),+ $(,)?) => {
        impl<F, R, $($ty),+> CommandHandler<($($ty,)+), SyncCommandHandlerKind> for F
        where
            F: Fn($($ty),+) -> R + Send + Sync + 'static,
            R: CommandOutput + Send + 'static,
            $(
                $ty: FromCommandArg + Send + 'static,
            )+
        {
            fn into_dyn(self) -> DynCommandHandleFn {
                Arc::new(move |message, store| {
                    $(
                        let $var = <$ty as FromCommandArg>::from_command_arg(message, store);
                    )+
                    let result = (self)($($var),+);
                    Box::pin(async move { CommandOutput::into_output(result) })
                })
            }
        }

        impl<F, Fut, R, $($ty),+> CommandHandler<($($ty,)+), AsyncCommandHandlerKind> for F
        where
            F: Fn($($ty),+) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = R> + Send + 'static,
            R: CommandOutput + Send + 'static,
            $(
                $ty: FromCommandArg + Send + 'static,
            )+
        {
            fn into_dyn(self) -> DynCommandHandleFn {
                Arc::new(move |message, store| {
                    $(
                        let $var = <$ty as FromCommandArg>::from_command_arg(message, store);
                    )+
                    let fut = (self)($($var),+);
                    Box::pin(async move {
                        let result = fut.await;
                        CommandOutput::into_output(result)
                    })
                })
            }
        }
    };
}

impl_command_handler!();
impl_command_handler!(A1 => a1);
impl_command_handler!(A1 => a1, A2 => a2);
impl_command_handler!(A1 => a1, A2 => a2, A3 => a3);
impl_command_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4);
impl_command_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5);
impl_command_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6);
impl_command_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6, A7 => a7);
impl_command_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6, A7 => a7, A8 => a8);

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
