use dioxus::prelude::*;

#[inline_props]
pub fn Input<'a>(
    cx: Scope,
    name: &'a str,
    field_type: &'a str,
    onchange: EventHandler<'a, Event<FormData>>,
) -> Element<'a> {
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
                name: "{name}",
                onchange: |e| { onchange.call(e) },
                r#type: "{field_type}"
            }
        }
    )
}
