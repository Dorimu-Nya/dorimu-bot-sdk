mod context_injection;
mod command;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Command 宏
///
/// 用于定义命令处理函数。宏会自动：
/// 1. 扫描函数参数，识别 State<T> 依赖
/// 2. 生成从容器中获取依赖的代码
/// 3. 注册命令到全局命令表
///
/// # 用法
/// ```rust
/// #[command("/ping")]
/// fn ping() -> ReplyingMessage {
///     ReplyingMessage::Text("Pong!".to_string())
/// }
///
/// #[command("/status")]
/// async fn status(State(app): State<App>) -> ReplyingMessage {
///     // app 会自动从容器中注入
///     ReplyingMessage::Text(format!("Bot ID: {}", app.config.app_id))
/// }
/// ```
#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as command::CommandArgs);
    let func = parse_macro_input!(input as syn::ItemFn);

    command::command_impl(args, func).into()
}
