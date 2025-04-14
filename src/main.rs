use dioxus::prelude::*;

// add files listed in the components.rs
mod components;

// import Nav component from /src/components/hero.rs
use components::hero::Nav;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        // imported components
        Nav {}
    }
}
