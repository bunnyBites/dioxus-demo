use dioxus::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};

use crate::components::input::Input;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserObj {
    username: Vec<String>,
    password: Vec<String>,
}

pub fn LoginForm(cx: Scope) -> Element {
    let username = use_state(cx, || String::default());
    let password = use_state(cx, || String::default());

    let onsubmit = move |event: Event<FormData>| {
        event.stop_propagation();
        info!("username: {}, password: {}", username, password);
    };

    let onchange_username = move |event: Event<FormData>| {
        username.set(event.value.clone());
    };

    let onchange_password = move |event: Event<FormData>| {
        password.set(event.value.clone());
    };

    render!(
        form {
            onsubmit: onsubmit,
            Input {
                name: "username",
                field_type: "text",
                onchange: onchange_username,
            }
            Input {
                name: "password",
                field_type: "password",
                onchange: onchange_password,
            }
            button {
                class: "btn btn-info w-100",
                r#type: "submit",
                "Submit"
            }
        }
    )
}
