#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::components::AddGiver;
use crate::{Participant, SecretSatan};

#[component]
pub fn App() -> Element {
    use_context_provider(|| SecretSatan::default());
    let secret_satan = use_context::<SecretSatan>();
    let mut givers = use_signal(|| secret_satan.participants.clone());

    rsx! {
        div {
            form {
                // for index in 1..= givers.read().len() + 1 {
                for index in 1..= givers.read().len() + 1 {
                    AddGiver { index }
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        info!("adding participant");
                        givers.write().push(Participant::default());
                    },
                    "Add another participant"
                }
                button {
                    r#type: "submit",
                    "Submit"
                }
            }
        }
    }
}