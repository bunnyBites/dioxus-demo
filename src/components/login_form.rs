use dioxus::prelude::*;

use crate::components::input::Input;

pub fn LoginForm(cx: Scope) -> Element {
    let onsubmit = |event: Event<FormData>| {
        event.stop_propagation();
        log::info!("{:?}", event.values);
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
