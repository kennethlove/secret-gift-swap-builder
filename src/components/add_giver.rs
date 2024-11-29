#![allow(non_snake_case)]

use crate::{Participant, SecretSatan};
use dioxus::prelude::*;

#[component]
pub fn AddGiver(name_signal: Signal<String>, excluding_signal: Signal<String>) -> Element {
    rsx! {
        div {
            input {
                r#type: "text",
                name: "name",
                placeholder: "Enter a participant's name",
                value: name_signal.read().clone(),
                class: "block w-full bg-white text-gray-800 px-3 py-2 rounded-lg border-gray-700 border-2 mb-2",
                oninput: move |event| {
                    name_signal.set(event.value().clone());
                }
            }
            textarea {
                name: "excluded",
                placeholder: "Enter any excluded receivers, one per line",
                value: excluding_signal.read().clone(),
                class: "block w-full bg-white text-gray-800 px-3 py-2 rounded-lg border-gray-700 border-2 mb-2 field-sizing-content",
                oninput: move |event| {
                    excluding_signal.set(event.value().clone());
                }
            }
        }
    }
}
