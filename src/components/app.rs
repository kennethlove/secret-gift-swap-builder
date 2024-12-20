#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{AddGiver, ClearListModal, DeleteParticipantModal, GuestForm, GuestList, GuestListItem, ListOutput};
use crate::{use_persistent, Participant, SecretSatan, UsePersistent};

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
    use_context_provider(|| Signal::<Option<Participant>>::new(None));
    use_context_provider(|| Signal::<bool>::new(false));
    let selected_participant = use_context::<Signal<Option<Participant>>>();

    rsx! {
        div {
            class: "lg:w-2/3 mx-auto p-4",
            div {
                class: "w-full",
                h1 {
                    class: "text-3xl font-bold text-white mb-2",
                    "Secret Satan",
                    img {
                        src: "/android-chrome-512x512.png",
                        alt: "",
                        class: "size-8 inline-block ml-2"
                    }
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
            ClearListModal {}
            DeleteParticipantModal {}
        }
    }
}