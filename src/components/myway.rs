use dioxus::prelude::*;

#[component]
pub fn MyWay() -> Element {
    rsx! {
        h1 {
            class: "bg-white text-center text-red-800",
            "hello sunless!"
        }
    }
}
