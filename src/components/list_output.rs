use crate::{use_persistent, Participant, SecretSatan};
use dioxus::prelude::*;

#[component]
pub fn ListOutput() -> Element {
    let state = use_context::<Signal<SecretSatan>>();
    let giving_list = use_context::<Signal<Vec<Participant>>>();
    let selected_participant = use_context::<Signal<Option<Participant>>>();

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
                        class: "shadow-sm rounded-lg candy-cane-striped hover:shadow-lg",
                        div {
                            class: "h-full px-4 py-2 rounded-lg bg-gradient-to-br from-white from-65%",
                            h2 {
                                class: "text-2xl font-bold",
                                {participant.name.clone()}
                                span {
                                    class: "font-normal",
                                    " is giving to "
                                }
                                {participant.giving_to.clone().unwrap()}
                            }
                        }
                    }
                }
            }
        }
    }
}