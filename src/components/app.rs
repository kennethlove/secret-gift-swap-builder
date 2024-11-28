#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{AddGiver, GuestListItem};
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
            for mut participant in participants.clone().iter_mut() {
                div {
                    {participant.name.clone()}
                    ul {
                        for guest in participants.clone().iter().filter(|p| participant.name != p.name) {
                            GuestListItem { guest: guest.clone().name, participant: participant.clone() }
                        }
                    }
                }
            }
        }
        div {
            form {
                onsubmit: move |event| {
                    let mut participant = Participant::default();
                    participant.name = name_signal.read().clone();
                    if participant.name.is_empty() {
                        return;
                    }
                    participant.excluding = excluding_signal.read().clone().split('\n').map(|name| name.trim().to_string()).collect();
                    let mut participants = santana.get().participants.clone();
                    participants.push(participant);
                    santana.set(SecretSatan { participants });

                    name_signal.set("".to_string());
                    excluding_signal.set("".to_string());
                },
                AddGiver { name_signal, excluding_signal}
                button {
                    r#type: "submit",
                    "Add"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        santana.set(SecretSatan::default());
                    },
                    "Reset"
                }
            }
        }
        div {
            button {
                r#type: "button",
                onclick: move |_| {
                    let participants = santana.get().assign_participants();
                    giving_list.set(participants.clone().expect("failed to assign participants"));
                    dioxus_logger::tracing::info!("{:?}", participants);
                },
                "Calculate gift giving list"
            }

            textarea {
                readonly: true,
                value: giving_list.read().iter().map(|p| format!("{} -> {}", p.clone().name, p.clone().giving_to.unwrap())).collect::<Vec<String>>().join("\n")
            }

        }
    }
}