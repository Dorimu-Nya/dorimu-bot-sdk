use std::collections::HashSet;

#[derive(Clone)]
pub struct AccessControl {
    allowlist: HashSet<String>,
    denylist: HashSet<String>,
}

impl AccessControl {
    pub fn from_env() -> Self {
        let allowlist = parse_list(std::env::var("BOT_ALLOW_OPENIDS").unwrap_or_default().as_str());
        let denylist = parse_list(std::env::var("BOT_DENY_OPENIDS").unwrap_or_default().as_str());
        Self { allowlist, denylist }
    }

    pub fn allowed(&self, openid: &str) -> bool {
        // 黑名单优先；白名单为空表示全部允许。
        if self.denylist.contains(openid) {
            return false;
        }
        if self.allowlist.is_empty() {
            return true;
        }
        self.allowlist.contains(openid)
    }
}

fn parse_list(raw: &str) -> HashSet<String> {
    raw.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
