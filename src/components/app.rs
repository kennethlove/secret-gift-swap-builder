#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{AddGiver, GuestForm, GuestList, GuestListItem, ListOutput};
use crate::{SecretSatan, use_persistent, Participant, UsePersistent};

fn get_saved_state(storage: UsePersistent<SecretSatan>) -> Signal<SecretSatan> {
    let mut state = SecretSatan::default();
    if !storage.get().participants.is_empty() {
        state = storage.get();
    }
    Signal::new(state)
}

#[component]
pub fn App() -> Element {
    let storage = use_persistent("satan", || SecretSatan::default());
    use_context_provider(|| get_saved_state(storage));
    use_context_provider(|| Signal::<Vec<Participant>>::new(vec![]));

    rsx! {
        div {
            div {
                class: "container mx-auto sm:w-full lg:w-3/4 mt-4",
                h1 {
                    class: "text-3xl font-bold text-white mb-2",
                    "Secret Satan"
                }
                div {
                    class: "flex flex-col gap-4 flex-wrap justify-stretch items-start",
                    GuestForm {}
                    GuestList {}
                }
                ListOutput {}

                p {
                    class: "text-white text-sm my-4 text-center",
                    "Made with ❤️ by ",
                    a {
                        href: "https://thekennethlove.com",
                        class: "underline",
                        "klove"
                    },
                    "."
                }
            }
        }
    }
}