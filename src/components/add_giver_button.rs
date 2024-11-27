#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{Participant, SecretSatan};

#[component]
pub fn AddGiverButton() -> Element {
    let mut satan = use_context::<Signal<SecretSatan>>();

    rsx! {
        button {
            r#type: "button",
            onclick: move |_| {
                satan.write().participants.push(Participant::default());
            },
            "Add Participant"
        }
    }
}