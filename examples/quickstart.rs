use qqbot_sdk::models::message::{
    Action, ActionType, Keyboard, KeyboardButton, KeyboardContent, KeyboardRow, MessageMarkdown,
    Permission, PermissionType, RenderData,
};
use qqbot_sdk::ReplyingMessage::Text;
use qqbot_sdk::{run_application, AppConfig, Context, CredentialConfig, ReplyingMessage};
use qqbot_sdk_macros::command;
use std::sync::atomic::{AtomicI16, Ordering};
struct CustomContext {
    pub value: AtomicI16,
}

struct HelloCmd {
    location: String,
}

impl CustomContext {
    fn new() -> Self {
        Self {
            value: AtomicI16::new(0),
        }
    }

    fn plus(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }
}

impl HelloCmd {
    fn say_hi(&self) -> ReplyingMessage {
        Text(String::from("Hi from ") + self.location.as_str())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let earth_hi = HelloCmd {
        location: "Earth".to_string(),
    };
    let moon_hi = HelloCmd {
        location: "Moon".to_string(),
    };

    let config = AppConfig::new()
        .credential(CredentialConfig {
            app_id: "".to_string(),
            secret: "".to_string(),
        })
        .bind_addr("0.0.0.0:3000")
        .webhook_path("/webhook")
        .prod_url_override("https://sandbox.api.sgroup.qq.com")
        .with_context(Context::new(CustomContext::new()))
        .with_command("/hi1", move || earth_hi.say_hi())
        .with_command("/hi2", move || moon_hi.say_hi());

    run_application(config).await
}

#[command("/ping")]
fn ping() -> ReplyingMessage {
    Text(String::from("Pong!"))
}
#[command("/im")]
fn asd(msg: Option<Vec<String>>) -> ReplyingMessage {
    if let Some(msg) = msg {
        Text(String::from("Hi! ") + msg[1..].join(" ").as_str())
    } else {
        Text(String::from("I can't know your name."))
    }
}

#[command("/couting")]
fn counting(context: Context<CustomContext>) -> ReplyingMessage {
    let v = context.value.load(Ordering::SeqCst);
    context.plus();
    Text(String::from("Current ") + String::from(v.to_string()).as_str())
}

#[command("/markdown")]
fn markdown() -> ReplyingMessage {
    fn create_default_action() -> Action {
        Action {
            action_type: ActionType::Callback,
            permission: Permission {
                permission_type: PermissionType::AdminOnly,
                specify_user_ids: None,
                specify_role_ids: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            },
            data: "data".to_string(),
            reply: None,
            enter: None,
            anchor: None,
            click_limit: Some(10),
            at_bot_show_channel_list: Some(true),
            unsupport_tips: "兼容文本".to_string(),
        }
    }

    ReplyingMessage::Markdown(MessageMarkdown {
        content: Some(
            "
            # 一号标题
            ## 二号标题
            正文

            **加粗**
            __下划线加粗__
            _斜体_
            *星号斜体*
            ***加粗斜体***
            ~~删除线~~

            欢迎来到：[🔗腾讯网](https://www.qq.com)
            文档可以访问<https://doc.qq.com>

            "
            .to_string(),
        ),
        custom_template_id: None,
        params: None,
        keyboard: Some(Keyboard {
            content: KeyboardContent {
                rows: vec![
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("1".to_string()),
                                render_data: RenderData {
                                    label: "⬅️上一页".to_string(),
                                    visited_label: "⬅️上一页".to_string(),
                                    style: None,
                                },
                                action: create_default_action(),
                            },
                            KeyboardButton {
                                id: Some("2".to_string()),
                                render_data: RenderData {
                                    label: "➡️下一页".to_string(),
                                    visited_label: "➡️下一页".to_string(),
                                    style: None,
                                },
                                action: create_default_action(),
                            },
                        ],
                    },
                    KeyboardRow {
                        buttons: vec![KeyboardButton {
                            id: Some("3".to_string()),
                            render_data: RenderData {
                                label: "📅 打卡（5）".to_string(),
                                visited_label: "📅 打卡（5）".to_string(),
                                style: None,
                            },
                            action: create_default_action(),
                        }],
                    },
                ],
            },
        }),
    })
}
