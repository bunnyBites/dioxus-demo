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
    let onsubmit = |event: Event<FormData>| {
        event.stop_propagation();
        let value_str: String = serde_json::to_string(&event.values).unwrap();
        let user_object: UserObj = serde_json::from_str(&value_str).unwrap();

        let prepared_user = User {
            username: user_object.username.into_iter().next().expect("The vector was empty"),
            password: user_object.password.into_iter().next().expect("The vector was empty"),
        };

        info!("{:?}", prepared_user);
    };

    render!(
        form {
            onsubmit: onsubmit,
            Input { name: "username", field_type: "text" }
            Input { name: "password", field_type: "password" }
            button {
                class: "btn btn-info w-100",
                r#type: "submit",
                "Submit"
            }
        }
    )
}
