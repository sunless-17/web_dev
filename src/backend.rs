use dioxus::prelude::*;

#[server]
pub async fn hello_world() -> Result<String, ServerFnError> {
  Ok("hello world!".to_string())
}
