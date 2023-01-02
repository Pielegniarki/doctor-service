use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "value")]
pub enum JsResponse<'a, T> {
    Ok(T),
    Err(&'a str)
}