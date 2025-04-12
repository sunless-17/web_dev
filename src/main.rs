use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod components;

use components::hero::Hero;
use components::myway::MyWay;

fn main() {
    dioxus::launch(App);
}

// default
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}
        MyWay {}
        h1 {
            class: "bg-white text-center text-red-800",
            "hello sunless!"
        }
    }
}
