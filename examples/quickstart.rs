use qqbot_sdk::{run_application, CommonMessage};
use qqbot_sdk_macros::command;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_application().await
}

#[command("/ping")]
fn asd(msg: Option<Vec<&str>>) {
    println!("Hello, world!. {:?}", msg);
}