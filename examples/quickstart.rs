use qqbot_sdk::{run_application, CommonMessage, ReplyingMessage};
use qqbot_sdk_macros::command;
use qqbot_sdk::ReplyingMessage::Text;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_application().await
}

#[command("/ping")]
fn asd(msg: Option<Vec<&str>>) -> ReplyingMessage {
    Text(String::from("Pong!"))
}
