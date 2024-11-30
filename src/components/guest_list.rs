use dioxus::prelude::*;

use crate::{components, use_persistent, Participant, SecretSatan};
use components::GuestListItem;

#[component]
pub fn GuestList() -> Element {
    let state = use_context::<Signal<SecretSatan>>();
    let participants = state.read().participants.clone();

    rsx! {
        div {
            class: "grid grid-row grid-wrap gap-4 grid-cols-1 sm:grid-cols-2 md:grid-cols-3 w-full",
            for mut participant in participants.clone().iter_mut() {
                div {
                    class: "candy-cane-striped shadow-sm hover:shadow-lg rounded-lg",
                    div {
                        class: "px-4 py-2 rounded-lg justify-stretch h-full bg-gradient-to-br from-white from-65%",
                        h2 {
                            class: "text-2xl font-bold",
                            {participant.name.clone()}
                        }
                        h3 {
                            class: "text-sm font-semibold select-none",
                            "can't give to"
                        }
                        ul {
                            class: "select-none",
                            for guest in participants.clone().iter().filter(|p| participant.name != p.name) {
                                GuestListItem { guest: guest.clone().name, participant: participant.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}