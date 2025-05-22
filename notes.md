# learning dioxus
- import `dioxus::prelude::*`
- `rsx!` returns an element
- `element { attribute: "value", "text"}`
- add css files to the `Dioxus.toml`
- props = parameters
- components = functions returning elements from rsx!
- if statement can be used in rsx! (on elements and attributes)
- use tracing for logging in the dioxus cli
- routers are done using enums and Routable macros, remember to include 404, use id or article titles based on the database
- `Link { to: Route::hi{}, "word"}`, navigator alternative to <a>
- `img {src: "link.png", height: "30px", width: "20px"}`

# forms
- `oninput` event store data in a signal

## functional programming
- for loop usable but focus on FP
- put data in a vec
- `.iter().map(|item| rsx!{ h1 {"{item}"} })`

# mini projects
- onclick closures to increment count + setting count as a `use_signal(||0)`

# quest
- components (navbar, body)
- backend (axum hello world + surreal)
