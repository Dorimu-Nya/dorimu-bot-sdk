use qqbot_sdk::ReplyingMessage::Text;
use qqbot_sdk::{run_application, AppConfig, CredentialConfig, ReplyingMessage};
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

#[command("/ping")]
fn asd(msg: Option<Vec<&str>>) -> ReplyingMessage {
    Text(String::from("Pong!"))
}
