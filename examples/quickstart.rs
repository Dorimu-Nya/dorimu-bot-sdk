use std::sync::atomic::{AtomicI16, Ordering};
use qqbot_sdk::ReplyingMessage::Text;
use qqbot_sdk::{run_application, AppConfig, Context, CredentialConfig, ReplyingMessage};
use qqbot_sdk_macros::command;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig {
        credential: CredentialConfig {
            app_id: "".to_string(),
            secret: "".to_string(),
        },
        ..Default::default()
    };

    run_application(config).await
}
struct CustomContext {
    pub value: AtomicI16
}
impl CustomContext {
    fn new() -> Self {
        Self { value: AtomicI16::new(0) }
    }

    fn plus(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }
}
#[command("/ping")]
fn ping() -> ReplyingMessage {
    Text(String::from("Pong!"))
}
#[command("/im")]
fn asd(msg: Option<Vec<&str>>) -> ReplyingMessage {
    if let Some(msg) = msg {
        Text(String::from("Hi! ") + msg[1..].join(" ").as_str())
    } else {
        Text(String::from("I can't know your name."))
    }
}

#[command("/couting")]
fn counting(Context(context): Context<CustomContext>) -> ReplyingMessage {
    let v = context.value.load(Ordering::SeqCst);
    context.plus();
    Text(String::from("Current ") + String::from(v.to_string()).as_str())
}