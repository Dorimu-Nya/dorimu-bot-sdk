# qqbot_sdk

一个正在开发中的对接支持 QQ机器人 官方API Webhook 框架，目标是一键开箱，快速使用。

## 宏的使用

### command

用于接收任何场景（私聊，群聊...) 的消息

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
- [ ] 非宏方式的存储command

## 考虑/计划中/设想的未来目标
- 提供配置读取
- 其他的还没想好
