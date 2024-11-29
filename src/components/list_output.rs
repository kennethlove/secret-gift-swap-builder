use dioxus::prelude::*;
use crate::{use_persistent, Participant, SecretSatan};

#[component]
pub fn ListOutput() -> Element {
    let state = use_context::<Signal<SecretSatan>>();
    let giving_list = use_context::<Signal<Vec<Participant>>>();

    let participants = state.read().participants.clone();

    let mut classes = vec!["mt-4", "flex", "flex-row", "flex-wrap", "gap-4"];
    if giving_list.read().is_empty() {
        classes.push("hidden");
    }

    rsx! {
        div {
            class: classes.join(" "),
            h2 {
                class: "text-2xl font-bold w-full text-white",
                "Gift Giving List"
            }

            for participant in giving_list.read().iter() {
                div {
                    class: "px-4 py-2 shadow-md bg-white rounded-lg justify-stretch",
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