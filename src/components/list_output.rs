use dioxus::prelude::*;
use crate::{use_persistent, Participant, SecretSatan};

#[component]
pub fn ListOutput() -> Element {
    let state = use_context::<Signal<SecretSatan>>();
    let giving_list = use_context::<Signal<Vec<Participant>>>();

    let participants = state.read().participants.clone();

    rsx! {
        div {
            class: if giving_list.read().is_empty() { "hidden mt-4" } else { "mt-4" },
            h2 {
                class: "text-2xl font-bold w-full text-white",
                "Gift Giving List"
            }

            div {
                class: "grid grid-row grid-wrap gap-4 w-full mt-4 sm:grid-cols-2 md:grid-cols-3",
                for participant in giving_list.read().iter() {
                    div {
                        class: "px-4 py-2 shadow-md bg-white rounded-lg",
                        h2 {
                            class: "text-2xl font-bold",
                            {participant.name.clone()}
                            " is giving to "
                            {participant.giving_to.clone().unwrap()}
                        }
                    }
                }
            }
        }
    }
}