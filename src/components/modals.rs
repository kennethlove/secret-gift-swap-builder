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
                        class: "relative transform overflow-hidden",
                        div {
                            class: "mx-auto bg-white border border-orange-500 rounded-xl p-4",
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
                                    onclick: move |event| {
                                        state.write().remove_participant(&selected.clone());
                                        storage.set(SecretSatan { participants: state.read().participants.clone() });
                                        participant.set(None);
                                    },
                                    "Yes"
                                }
                                button {
                                    onclick: move |event| {
                                        participant.set(None);
                                    },
                                    "No"
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
                        class: "relative transform overflow-hidden",
                        div {
                            class: "mx-auto bg-white border border-orange-500 rounded-xl p-4",
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
                                    onclick: move |event| {
                                        state.write().participants.clear();
                                        storage.set(SecretSatan::default());
                                        open.set(false);
                                    },
                                    "Yes"
                                }
                                button {
                                    onclick: move |event| {
                                        open.set(false);
                                    },
                                    "No"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
