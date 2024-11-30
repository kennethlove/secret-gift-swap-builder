use crate::{use_persistent, Participant, SecretSatan};
use dioxus::prelude::*;

#[component]
pub fn DeleteParticipantModal() -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());
    let mut state = use_context::<Signal<SecretSatan>>();
    let mut participant = use_context::<Signal<Option<Participant>>>();
    let mut name = String::new();
    let mut selected = Participant::default();
    if participant.read().is_some() {
        selected = participant.read().clone().unwrap();
        name = selected.name.clone();
    }

    rsx! {
        dialog {
            open: !participant.read().is_none(),
            class: "relative z-10",
            role: "confirm",
            div { class: "fixed inset-0 bg-red-200/25 transition-opacity backdrop-blur-sm backdrop-grayscale" }
            div {
                class: "fixed inset-0 z-10 w-screen h-screen overflow-y-hidden",
                div {
                    class: "flex items-center gap-4 min-h-full justify-center",
                    div {
                        class: "relative transform overflow-hidden p-2 candy-cane-striped rounded-xl",
                        div {
                            class: "mx-auto bg-white border border-red-400 rounded-xl p-2",
                            div {
                                class: "flex-1",
                                h1 {
                                    class: "block font-medium text-gray-900",
                                    "Delete {name}?"
                                }
                                p {
                                    class: "mt-1 text-sm text-gray-700",
                                    "Do you want to remove {name} from the list?"
                                }
                            }
                            div {
                                class: "flex justify-end gap-4 mt-4",
                                button {
                                    r#type: "button",
                                    class: "bg-red-900 text-white px-3 py-2 rounded-lg border-red-700 border-2 cursor-danger whitespace-nowrap disabled:cursor-not-allowed disabled:opacity-50 hover:bg-red-600 cursor-pointer",
                                    onclick: move |event| {
                                        state.write().remove_participant(&selected.clone());
                                        storage.set(SecretSatan { participants: state.read().participants.clone() });
                                        participant.set(None);
                                    },
                                    "Yes"
                                }
                                button {
                                    r#type: "button",
                                    class: "group text-gray-800 bg-white px-1 py-2 rounded-lg border-gray-200 border-2 mr-2 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 calculate-button transition",
                                    onclick: move |event| {
                                        participant.set(None);
                                    },
                                    span {
                                        class: "bg-white py-1 px-1 rounded-md",
                                        "Nevermind"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ClearListModal() -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());
    let mut state = use_context::<Signal<SecretSatan>>();
    let mut open = use_context::<Signal<bool>>();


    rsx! {
        dialog {
            open: open,
            class: "relative z-10",
            role: "confirm",
            div { class: "fixed inset-0 bg-red-200/25 transition-opacity backdrop-blur-sm backdrop-grayscale" }
            div {
                class: "fixed inset-0 z-10 w-screen h-screen overflow-y-hidden",
                div {
                    class: "flex items-center gap-4 min-h-full justify-center",
                    div {
                        class: "relative transform overflow-hidden candy-cane-striped p-2 rounded-xl",
                        div {
                            class: "mx-auto bg-white border border-red-400 rounded-xl p-2",
                            div {
                                class: "flex-1",
                                h1 {
                                    class: "block font-medium text-gray-900",
                                    "Clear your list?"
                                }
                                p {
                                    class: "mt-1 text-sm text-gray-700",
                                    "Do you want to remove all of your participants?"
                                }
                            }
                            div {
                                class: "flex justify-end gap-4 mt-4",
                                button {
                                    r#type: "button",
                                    class: "bg-red-900 text-white px-3 py-2 rounded-lg border-red-700 border-2 cursor-danger whitespace-nowrap disabled:cursor-not-allowed disabled:opacity-50 hover:bg-red-600 cursor-pointer",
                                    onclick: move |event| {
                                        state.write().participants.clear();
                                        storage.set(SecretSatan::default());
                                        open.set(false);
                                    },
                                    "Yes"
                                }
                                button {
                                    r#type: "button",
                                    class: "group text-gray-800 bg-white px-1 py-2 rounded-lg border-gray-200 border-2 mr-2 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 calculate-button transition",
                                    onclick: move |event| {
                                        open.set(false);
                                    },
                                    span {
                                        class: "bg-white py-1 px-1 rounded-md",
                                        "Nevermind"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
