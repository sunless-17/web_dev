use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
    rsx! {
        h1 {
         class: "text-sm",
         "hello world!"
        }
    }
}
