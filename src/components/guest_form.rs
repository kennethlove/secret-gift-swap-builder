use dioxus::prelude::*;

use crate::{components, use_persistent, Participant, SecretSatan};
use components::AddGiver;

#[component]
pub fn GuestForm() -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());
    let mut state = use_context::<Signal<SecretSatan>>();
    let mut modal_open = use_context::<Signal<bool>>();

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
                    class: "flex flex-row flex-wrap gap-2 w-full justify-between",
                    div {
                        class: "md:w-1/2 sm:w-full",
                        button {
                            r#type: "submit",
                            class: "group text-gray-800 bg-white px-1 py-2 rounded-lg border-gray-200 border-2 mr-2 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 calculate-button transition",
                            span {
                                class: "bg-white py-1 px-1 rounded-md",
                                "Add participant"
                            }
                        }
                        button {
                            r#type: "button",
                            class: "group text-gray-800 bg-white px-1 py-2 rounded-lg border-gray-200 border-2 mr-2 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 calculate-button transition",
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
                            span {
                                class: "bg-white py-1 px-1 rounded-md",
                                "Calculate gift giving list"
                            }
                        }
                    }
                    div {
                        class: "flex flex-row flex-nowrap gap-1 justify-end",
                        div {
                            class: "max-w-max",
                            button {
                                r#type: "button",
                                class: "bg-red-900 text-white px-3 py-2 rounded-lg border-red-700 border-2 cursor-danger whitespace-nowrap disabled:cursor-not-allowed disabled:opacity-50 hover:bg-red-600",
                                disabled: participants.is_empty(),
                                onclick: move |_| {
                                    modal_open.set(true);
                                },
                                "Reset All"
                            }
                        }
                        div {
                            class: "text-xs text-gray-100 font-semibold flex-shrink w-fit text-pretty pt-1",
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