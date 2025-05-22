use dioxus::{logger::tracing::info, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

pub mod backend;

fn main() {
  dioxus::launch(App);
}

#[component]
fn app() -> element {
  rsx! {
      document::link { rel: "icon", href: favicon }
      document::link { rel: "stylesheet", href: main_css }
      hero {}
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
