use dioxus::prelude::*;

use crate::components::login_form::LoginForm;

pub fn Login(cx: Scope) -> Element {
    render!(
        div {
            class: "container-fluid",
            div {
                class: "row align-items-center justify-content-center min-vh-100",
                div {
                    class: "col-sm-4",
                    div {
                        class: "card",
                        div {
                            class: "card-body",
                            LoginForm {}
                        }
                    }
                }
            }
        }
    )
}
