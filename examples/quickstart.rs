use std::sync::atomic::{AtomicI16, Ordering};
use qqbot_sdk::ReplyingMessage::Text;
use qqbot_sdk::{run_application, AppConfig, Context, CredentialConfig, ReplyingMessage};
use qqbot_sdk_macros::command;

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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::new()
        .credential(CredentialConfig {
            app_id: "".to_string(),
            secret: "".to_string(),
        })
        .bind_addr("0.0.0.0:3000")
        .webhook_path("/webhook")
        .prod_url_override("https://sandbox.api.sgroup.qq.com")
        .with_context(Context::new(CustomContext::new()))
        ;

    run_application(config).await
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
fn counting(context: Context<CustomContext>) -> ReplyingMessage {
    let v = context.value.load(Ordering::SeqCst);
    context.plus();
    Text(String::from("Current ") + String::from(v.to_string()).as_str())
}
