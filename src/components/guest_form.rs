use dioxus::prelude::*;

use crate::{components, use_persistent, Participant, SecretSatan};
use components::AddGiver;

#[component]
pub fn GuestForm() -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());
    let mut name_signal = use_signal(|| "".to_string());
    let mut excluding_signal = use_signal(|| "".to_string());
    let mut giving_list: Signal<Vec<Participant>> = use_signal(|| vec![]);

    let mut participants = storage.get().participants.clone();

    rsx! {
      div {
            class: "w-full",
            form {
                onsubmit: move |event| {
                    let mut participant = Participant::default();
                    participant.name = name_signal.read().clone();
                    if participant.name.is_empty() {
                        return;
                    }
                    participant.excluding = excluding_signal.read().clone().split('\n').map(|name| name.trim().to_string()).collect();
                    let mut participants = storage.get().participants.clone();
                    participants.push(participant);
                    storage.set(SecretSatan { participants });
                    name_signal.set("".to_string());
                    excluding_signal.set("".to_string());
                },
                AddGiver { name_signal, excluding_signal}
                div {
                    class: "flex flex-row gap-2 w-full",
                    div {
                        class: "w-1/2",
                        button {
                            r#type: "submit",
                            class: "bg-green-500 text-white px-3 py-2 rounded-lg border-green-700 border-2 mr-2 cursor-pointer",
                            "Add"
                        }
                        button {
                            r#type: "button",
                            class: "text-slate-800 bg-green-300 px-3 py-2 rounded-lg border-green-700 border-2 mr-2 cursor-pointer",
                            onclick: move |_| {
                                let participants = storage.get().assign_participants();
                                giving_list.set(participants.clone().expect("failed to assign participants"));
                                dioxus_logger::tracing::info!("{:?}", participants);
                            },
                            "Calculate gift giving list"
                        }
                    }
                    div {
                        class: "flex flex-row text-pretty gap-1",
                        div {
                            button {
                                r#type: "button",
                                class: "bg-gray-500 text-white px-3 py-2 rounded-lg border-red-700 border-2 cursor-pointer whitespace-nowrap",
                                onclick: move |_| {
                                    storage.set(SecretSatan::default());
                                },
                                "Reset All"
                            }
                        }
                        div {
                            class: "text-xs text-gray-900 font-semibold",
                            "This will reset ",
                            i { "all" },
                            " participants and their exclusions."
                        }
                    }
                }
            }
        }
    }
}