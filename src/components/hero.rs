use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        body {
            class: "bg-white"
        }
        h1 {
            class: "text-red-50",
            "Hello World"
        }
        div {
            class: "w-full bg-red-800",
            h1 {
                class: "text-center text-9xl font-black",
                "hi"
            }
        }
        p { "yoo" }
    }
}
