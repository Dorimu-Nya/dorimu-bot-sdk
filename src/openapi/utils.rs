use crate::{Error, Result};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

/// 拼接 base URL 与 API path。
pub(crate) fn join_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{}/{}", base, path)
}

/// 读取路径模板，缺失时返回明确错误。
pub(crate) fn require_path(path: &Option<String>, name: &str) -> Result<String> {
    path.clone()
        .ok_or_else(|| Error::Other(format!("missing openapi path template: {name}")))
}

/// 渲染路径模板并对路径参数进行百分号编码。
pub(crate) fn render_path(template: &str, params: &[(&str, &str)]) -> Result<String> {
    let mut out = template.to_string();
    for (key, value) in params {
        let needle = format!("{{{}}}", key);
        if !out.contains(&needle) {
            return Err(Error::Other(format!(
                "path template missing placeholder: {needle}"
            )));
        }
        // 路径参数做百分号编码，避免 `/ ? # &` 等字符污染路由。
        let encoded = encode_path_segment(value);
        out = out.replace(&needle, &encoded);
    }
    Ok(out)
}

/// 追加 query 参数并进行安全编码。
pub(crate) fn append_query(path: String, params: &[(&str, Option<String>)]) -> String {
    let mut parts = Vec::new();
    for (key, value) in params {
        if let Some(v) = value {
            // query key/value 分别编码，避免拼接注入。
            let encoded_key = encode_query_component(key);
            let encoded_value = encode_query_component(v);
            parts.push(format!("{encoded_key}={encoded_value}"));
        }
    }
    if parts.is_empty() {
        return path;
    }
    let separator = if path.contains('?') { "&" } else { "?" };
    format!("{path}{separator}{}", parts.join("&"))
}

fn encode_path_segment(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

fn encode_query_component(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}
