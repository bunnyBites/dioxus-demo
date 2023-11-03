use dioxus::prelude::*;

use crate::components::input::Input;

pub fn LoginForm(cx: Scope) -> Element {
    render!(
        form {
            Input { name: "username".to_string() }
            Input { name: "password".to_string() }
            button {
                class: "btn btn-primary w-100",
                "Submit"
            }
        }
    )
}
