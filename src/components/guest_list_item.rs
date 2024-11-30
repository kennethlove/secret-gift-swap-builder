use crate::{use_persistent, Participant, SecretSatan};
use dioxus::prelude::*;

#[component]
pub fn GuestListItem(guest: String, participant: Participant) -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());
    let mut state = use_context::<Signal<SecretSatan>>();

    rsx! {
        li {
            input {
                r#type: "checkbox",
                name: format!("{}-exclude", participant.name.replace(" ", "-")),
                value: guest.clone(),
                checked: participant.excluding.contains(&guest),
                class: "mr-2 rounded text-red-800 ",
                onchange: move |event| {
                    let mut participants = state.read().clone().participants;
                    let mut participant = participants.iter_mut().find(|p| p.name == participant.name).unwrap();

                    if event.checked() {
                        participant.excluding.push(guest.clone());
                    } else {
                        participant.excluding.retain(|name| name != &guest);
                    }

                    state.write().participants = participants.clone();
                    storage.set(SecretSatan { participants: participants.clone() });
                }
            }
            {guest.clone()}
        }
    }
}

