use std::collections::HashMap;

pub fn html_decode(
    value: &tera::Value,
    _fields: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    match value {
        tera::Value::Null => Ok(tera::Value::Null),
        tera::Value::String(html) => Ok(tera::Value::String(
            html_escape::decode_html_entities(&html).to_string(),
        )),
        _ => Err(tera::Error::msg("found invalid type. expected html string")),
    }
}
