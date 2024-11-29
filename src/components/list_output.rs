use dioxus::prelude::*;
use crate::{use_persistent, Participant, SecretSatan};

#[component]
pub fn ListOutput() -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());
    let mut giving_list: Signal<Vec<Participant>> = use_signal(|| vec![]);

    let participants = storage.get().participants.clone();

    rsx! {
        div {
            class: "mt-4",

            textarea {
                class: "block w-full bg-white text-gray-800 px-3 py-2 rounded-lg border-gray-700 border-2 mt-2 field-sizing-content",
                readonly: true,
                value: giving_list.read().iter().map(|p| format!("{} -> {}", p.clone().name, p.clone().giving_to.unwrap())).collect::<Vec<String>>().join("\n")
            }

        }
    }
}