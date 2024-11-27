#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::{AddGiver, AddGiverButton};
use crate::SecretSatan;

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(SecretSatan::default()));
    let satan = use_context::<Signal<SecretSatan>>();

    rsx! {
        div {
            form {
                onsubmit: |event| {
                    dioxus_logger::tracing::info!("Submitting form");
                    for (field, value) in event.data().values() {
                        for (idx, value) in value.iter().enumerate() {
                            dioxus_logger::tracing::info!("{}: {:?}", field, value);
                        }
                    }
                },
                for _ in 1..= satan.read().participants.len() + 1 {
                    AddGiver {}
                }

                AddGiverButton {}

                button {
                    r#type: "submit",
                    "Submit"
                }
            }
        }
    }
}