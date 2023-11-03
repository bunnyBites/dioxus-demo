use dioxus::prelude::*;

#[inline_props]
pub fn Input(cx: Scope, name: String) -> Element {
    let field_id = format!("field-{}", name);

    render!(
        div {
            class: "mb-3",
            label {
                class: "form-label",
                "{name}"
            }
            input {
                id: "{field_id}",
                class: "form-control",
                name: "{name}"
            }
        }
    )
}
