#![allow(non_snake_case)]

use crate::{Participant, SecretSatan};
use dioxus::prelude::*;

#[component]
pub fn AddGiver(name_signal: Signal<String>, excluding_signal: Signal<String>) -> Element {
    let _satan = use_context::<Signal<SecretSatan>>();

    rsx! {
        div {
            input {
                r#type: "text",
                name: "name",
                placeholder: "Enter a participant's name",
                value: name_signal.read().clone(),
                oninput: move |event| {
                    name_signal.set(event.value().clone());
                }
            }
            textarea {
                name: "excluded",
                placeholder: "Enter any excluded receivers, one per line",
                value: excluding_signal.read().clone(),
                oninput: move |event| {
                    excluding_signal.set(event.value().clone());
                }
            }
        }
    }
}
