use dioxus::prelude::*;

use crate::{components, use_persistent, Participant, SecretSatan};
use components::GuestListItem;

#[component]
pub fn GuestList() -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());
    let participants = storage.get().participants.clone();

    rsx! {
        div {
            class: "grid grid-row grid-wrap gap-4 grid-cols-1 sm:grid-cols-2 md:grid-cols-3 w-full",
            for mut participant in participants.clone().iter_mut() {
                div {
                    class: "px-4 py-2 shadow-md bg-white rounded-lg justify-stretch",
                    h2 {
                        class: "text-2xl font-bold",
                        {participant.name.clone()}
                    }
                    ul {
                        for guest in participants.clone().iter().filter(|p| participant.name != p.name) {
                            GuestListItem { guest: guest.clone().name, participant: participant.clone() }
                        }
                    }
                }
            }
        }
    }
}