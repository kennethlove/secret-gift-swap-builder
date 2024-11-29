use dioxus::prelude::*;

use crate::{components, use_persistent, Participant, SecretSatan};
use components::AddGiver;

#[component]
pub fn GuestForm() -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());
    let mut state = use_context::<Signal<SecretSatan>>();

    let mut name_signal = use_signal(|| "".to_string());
    let mut excluding_signal = use_signal(|| "".to_string());
    let mut giving_list = use_context::<Signal<Vec<Participant>>>();

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

                    state.write().participants.push(participant.clone());
                    storage.set(SecretSatan { participants: state.read().participants.clone() });

                    name_signal.set("".to_string());
                    excluding_signal.set("".to_string());
                },
                AddGiver { name_signal, excluding_signal}
                div {
                    class: "flex flex-row gap-2 w-full justify-between",
                    div {
                        class: "w-1/2",
                        button {
                            r#type: "submit",
                            class: "bg-green-500 text-white px-3 py-2 rounded-lg border-green-700 border-2 mr-2 cursor-pointer",
                            "Add"
                        }
                        button {
                            r#type: "button",
                            class: "text-slate-800 bg-green-300 px-3 py-2 rounded-lg border-green-700 border-2 mr-2 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50",
                            disabled: participants.is_empty() || participants.len() < 3,
                            onclick: move |_| {
                                let participants = state.read().clone().assign_participants();
                                match participants {
                                    Ok(participants) => giving_list.set(participants.clone()),
                                    Err(_) => {
                                        giving_list.set(vec![]);
                                        eval("alert('There was an error calculating the gift giving list. Please try again.')");
                                    },
                                }
                            },
                            "Calculate gift giving list"
                        }
                    }
                    div {
                        class: "flex flex-row flex-nowrap gap-1 justify-end",
                        div {
                            class: "max-w-max",
                            button {
                                r#type: "button",
                                class: "bg-gray-500 text-white px-3 py-2 rounded-lg border-red-700 border-2 cursor-pointer whitespace-nowrap disabled:cursor-not-allowed disabled:opacity-50",
                                disabled: participants.is_empty(),
                                onclick: move |_| {
                                    state.write().participants.clear();
                                    storage.set(SecretSatan::default());
                                },
                                "Reset All"
                            }
                        }
                        div {
                            class: "text-xs text-gray-900 font-semibold flex-shrink w-fit text-pretty",
                            "This will reset ",
                            i { "all" },
                            " participants ",
                            br {},
                            "and their exclusions."
                        }
                    }
                }
            }
        }
    }
}