#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{AddGiver, GuestForm, GuestList, GuestListItem, ListOutput};
use crate::{SecretSatan, use_persistent, Participant};

#[component]
pub fn App() -> Element {
    let mut santana = use_persistent("satan", || SecretSatan::default());
    let mut name_signal = use_signal(|| "".to_string());
    let mut excluding_signal = use_signal(|| "".to_string());
    let mut giving_list: Signal<Vec<Participant>> = use_signal(|| vec![]);

    let mut participants = santana.get().participants.clone();

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
                    "Made with ❤️ by klove."
                }
            }
        }
    }
}