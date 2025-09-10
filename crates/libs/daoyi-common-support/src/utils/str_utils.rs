use std::collections::HashMap;
use strfmt::strfmt;

fn format_template_content_res(
    content: &str,
    params: &HashMap<String, String>,
) -> Result<String, strfmt::FmtError> {
    strfmt(content, params)
}

// 如果你想保持与 Java 相同的接口（返回 String 而不是 Result），可以这样：
pub fn format_template_content(content: &str, params: &HashMap<&str, String>) -> String {
    let params = params
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect::<HashMap<_, _>>();
    format_template_content_res(content, &params).unwrap_or_else(|_| content.to_string())
}
