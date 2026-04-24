# qqbot_sdk

一个正在开发中的对接支持 QQ机器人 官方API Webhook 框架，目标是一键开箱，快速使用。

## 消息指令

### command

用于接收任何场景（私聊，群聊...) 的消息

#### 使用宏

```rust
#[command("/ping")]
fn ping(msg: &dyn CommonMessage) {
    // Your biz logic...
}
```
可装填的参数:
- &dyn CommonMessage 抽象后的消息对象
- String 原始消息字符串
- Option<Vec<&str>> 将会被装入被空格分割后的消息
- Option<Vec<Attachment>> 消息附件

如果想要实现自己的转换，则需要自行实现 `FromCommonMessage`, 如:

```rust
impl<'a> FromCommonMessage<'a> for &'a YourStruct {
    fn from(req: &'a dyn CommonMessage) -> Self {
        YourStruct { 
            // Your coustructing ...
        }
    }
}
```

#### 不使用宏

除了 `#[command]`，也可以在 `AppConfig` 初始化时通过 `with_command` 注册。

```rust
use qqbot_sdk::{
    AppConfig, Context, CredentialConfig, ReplyingMessage,
};

struct CounterContext {
    value: std::sync::atomic::AtomicUsize,
}

fn ping() -> ReplyingMessage {
    ReplyingMessage::Text("Pong!".to_string())
}

fn echo(words: Option<Vec<String>>) -> ReplyingMessage {
    let content = words.unwrap_or_default().join(" ");
    ReplyingMessage::Text(content)
}

async fn count(ctx: Context<CounterContext>) -> ReplyingMessage {
    let current = ctx.value.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    ReplyingMessage::Text(format!("Current: {current}"))
}

let config = AppConfig::new()
    .credential(CredentialConfig {
        app_id: "YOUR_APP_ID".to_string(),
        secret: "YOUR_SECRET".to_string(),
    })
    .with_context(Context::new(CounterContext {
        value: std::sync::atomic::AtomicUsize::new(0),
    }))
    .with_command("/ping", ping)
    .with_command("/echo", echo)
    .with_command("/count", count);
```

手动注册时的参数提取规则：

- `Context<T>`: 从 `ContextStore` 注入
- `String` / `Option<String>`: 提取消息文本
- `Option<Vec<String>>`: 以空格切分消息文本
- `Option<Vec<Attachment>>`: 提取消息附件
- 返回值与 `#[command]` 一致，仍走 `CommandOutput` 统一转换

说明：

- 手动注册目前更适合使用拥有所有权的参数类型
- 借用类型（如 `Option<Vec<&str>>`、`&dyn CommonMessage`）建议继续使用 `#[command]`

## 上下文存储
类似于actix/axum的状态注入，可以存储像数据库连接池等对象。
首先需要在初始化AppConfig的时候使用 `with_context`

```rust
pub struct YourContext;

let config = AppConfig::new();
//Your other config...
let config = config.with_context(Context::new(YourContext));
```
可以在任意上方提及到的宏处理的方法使用，如：
```rust
#[command("/ping")]
fn has_context(context: Context<YourContext>) {
    // Your biz logic...
}
```

## 当前开发目标和进度

- [x] Webhook 事件的解析和处理函数
- [x] 使用宏 收集 处理消息指令、其他事件的函数和对应的 trait 生成
- [ ] open api 部分的代码指令提高和文档
- [x] 应用项目的启动参数的解析传递
- [ ] 其他事件的处理
- [x] 非宏方式的存储command

## 考虑/计划中/设想的未来目标
- 提供配置读取
- 其他的还没想好
