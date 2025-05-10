use dioxus::{logger::tracing::info, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: MAIN_CSS }
      Hero {}

  }
}

#[component]
fn Hero() -> Element {
  rsx! {
      div {
          id: "hero",
          // img { src: HEADER_SVG, id: "header" }
          div { id: "links", class: "bg-red-100",
            button { onclick: |_|info!("clicked!"), "Click Me!!" }
          }
      }
  }
}
