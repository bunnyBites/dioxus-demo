use dioxus::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};

use crate::components::input::Input;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

pub fn LoginForm(cx: Scope) -> Element {
    let user_info = use_state(cx, || User {
        username: String::default(),
        password: String::default(),
    });

    let onsubmit = move |event: Event<FormData>| {
        event.stop_propagation();
        info!("{:?}", user_info);
    };

    let onchange_username = move |event: Event<FormData>| {
        user_info.set(User {
            username: event.value.clone(),
            password: user_info.password.clone(),
        })
    };

    let onchange_password = move |event: Event<FormData>| {
        user_info.set(User {
            username: user_info.username.clone(),
            password: event.value.clone()
        });
    };

    render!(
        form {
            onsubmit: onsubmit,
            Input {
                name: "username",
                field_type: "text",
                onchange: onchange_username,
                value: user_info.username.clone(),
            }
            Input {
                name: "password",
                field_type: "password",
                onchange: onchange_password,
                value: user_info.username.clone(),
            }
            button {
                class: "btn btn-info w-100",
                r#type: "submit",
                "Submit"
            }
        }
    )
}
