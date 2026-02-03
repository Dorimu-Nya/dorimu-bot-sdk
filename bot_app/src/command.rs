#[derive(Debug, Clone)]
pub enum Command {
    Ping,
    Help,
    Get { key: String },
    Set { key: String, value: String },
    Unknown,
}

pub fn parse_command(prefix: &str, content: &str) -> Option<Command> {
    // 解析“前缀 + 指令”格式，例如 "/set key value"。
    if !content.starts_with(prefix) {
        return None;
    }
    let raw = content[prefix.len()..].trim();
    if raw.is_empty() {
        return Some(Command::Help);
    }
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let cmd = parts.first()?.to_lowercase();
    match cmd.as_str() {
        "ping" => Some(Command::Ping),
        "help" => Some(Command::Help),
        "get" => {
            let key = parts.get(1).map(|v| v.to_string());
            key.map(|k| Command::Get { key: k }).or(Some(Command::Unknown))
        }
        "set" => {
            if parts.len() < 3 {
                return Some(Command::Unknown);
            }
            let key = parts[1].to_string();
            let value = parts[2..].join(" ");
            Some(Command::Set { key, value })
        }
        _ => Some(Command::Unknown),
    }
}

pub fn help_text(prefix: &str) -> String {
    format!(
        "指令:\n{0}ping\n{0}help\n{0}get <key>\n{0}set <key> <value>",
        prefix
    )
}
