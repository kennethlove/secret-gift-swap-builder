#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::SecretSatan;

#[component]
pub fn AddGiver(index: usize) -> Element {
    let satan = use_context::<SecretSatan>();
    let mut givers = use_signal(|| satan.participants.clone());

    rsx! {
        div {
            input {
                r#type: "text",
                name: "name",
                id: "name",
                placeholder: "Enter a participant's name"
            }
            textarea {
                placeholder: "Enter any excluded receivers, one per line"
            }
            button {
                r#type: "button",
                onclick: move |_| {
                    givers.remove(index - 1);
                },
                "Remove"
            }
        }
    }
}