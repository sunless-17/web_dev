use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
    rsx! {
       nav {
         class: "bg-blue-500 py-4 px-6 flex items-center justify-between",
            div {
                h1 {
                    class:"text-sky-400 text-sm",
                    "My Awesome App"
                }
            }

            div {
                class: "flex items-center space-x-4 text-red-50",
                span { "sunless" }
                span { "light" }
            }
       }
    }
}
