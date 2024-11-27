#![allow(non_snake_case)]

use crate::{Participant, SecretSatan};
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn AddGiver() -> Element {
    let mut satan = use_context::<Signal<SecretSatan>>();
    let mut name_signal = use_signal(|| "".to_string());

    rsx! {
        div {
            input {
                r#type: "text",
                name: "name",
                placeholder: "Enter a participant's name",
                oninput: move |event| {
                    name_signal.set(event.value().clone());
                }
            }
            textarea {
                name: "excluded",
                placeholder: "Enter any excluded receivers, one per line"
            }
            button {
                r#type: "button",
                onclick: move |_| {
                    dioxus_logger::tracing::info!("Removing participant");
                    // let name = name_signal.read().clone();
                    // let index = satan.read().participants.iter().position(|p| p.name == name).unwrap_or(0);
                    // satan.write().participants.remove(index);
                },
                "Remove"
            }
        }
    }
}
