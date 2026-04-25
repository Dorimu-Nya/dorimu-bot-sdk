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
    let earth_hi = HelloCmd { location: "Earth".to_string() };
    let moon_hi = HelloCmd { location: "Moon".to_string() };
    
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
        .with_command("/hi2", move || moon_hi.say_hi())
    ;

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
