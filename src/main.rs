#![allow(non_snake_case)]
use dioxus::prelude::*;

mod components;
mod pages;

use crate::{components::appbar::Appbar, pages::login::Login};

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        // appbar section
        Appbar {}

        // login form section
        Login {}
    )
}
