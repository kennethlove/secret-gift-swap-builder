use dioxus::prelude::*;
use crate::{use_persistent, Participant, SecretSatan};

#[component]
pub fn GuestListItem(guest: String, participant: Participant) -> Element {
    let mut storage = use_persistent("satan", || SecretSatan::default());

    rsx! {
        li {
            input {
                r#type: "checkbox",
                name: format!("{}-exclude", participant.name.replace(" ", "-")),
                value: guest.clone(),
                checked: participant.excluding.contains(&guest),
                onchange: move |event| {
                    let mut participants = storage.get().participants;
                    let mut participant = participants.iter_mut().find(|p| p.name == participant.name).unwrap();

                    if event.checked() {
                        participant.excluding.push(guest.clone());
                    } else {
                        participant.excluding.retain(|name| name != &guest);
                    }

                    storage.set(SecretSatan { participants: participants.clone() });
                }
            }
            {guest.clone()}
        }
    }
}

