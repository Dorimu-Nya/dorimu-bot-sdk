# qqbot_sdk

一个正在开发中的对接支持 QQ机器人 官方API Webhook 框架，目标是一键开箱，快速使用。

## 宏的使用

### common_message

用于接收任何场景（私聊，群聊...) 的消息

```rust
#[command("/ping")]
fn ping(msg: &dyn CommonMessage) {
    // Your biz logic...
}
```
#### 可装填的参数:
- &dyn CommonMessage 抽象后的消息对象
- String 原始消息字符串
- Option<Vec<&str>> 将会被装入被空格分割后的消息
- Option<Vec<Attachment>> 消息附件

## 当前开发目标和进度

- [x] Webhook 事件的解析和处理函数
- [ ] 使用宏 收集 处理消息指令、其他事件的函数和对应的 trait 生成
- [ ] open api 部分的代码指令提高和文档
- [ ] 应用项目的启动参数的解析传递

## 考虑/计划中/设想的未来目标

- 提供 IoC + DI 去管理 HttpClient 和其他东西 ...
- 其他的还没想好