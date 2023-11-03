use dioxus::prelude::*;

pub fn Appbar(cx: Scope) -> Element {
    render!(
        nav {
            class: "navbar navbar-dark bg-info",
            div {
                class: "container-fluid",
                span {
                    class: "navbar-brand mb-0 h1",
                    "Bits Bunny"
                }
            }
        }
    )
}
