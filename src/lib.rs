pub mod components;

use std::collections::{HashMap, VecDeque};
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use rand::seq::SliceRandom;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct SecretSatan {
    pub participants: Vec<Participant>,
}

impl SecretSatan {
    pub fn new() -> SecretSatan {
        SecretSatan {
            participants: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }

    pub fn assign_participants(self) -> Result<Vec<Participant>, SecretSatanError> {
        let all_givers = self.participants.clone();

        let mut givers: HashMap<String, Participant> = HashMap::new();
        for giver in all_givers.iter() {
            givers.insert(giver.name.clone(), giver.clone());
        }

        for participant in all_givers.clone() {
            let mut participant = givers.get(&participant.name).unwrap().clone();
            let mut recipients = VecDeque::from(all_givers.clone());
            let rng = &mut rand::thread_rng();
            recipients.make_contiguous().shuffle(rng);

            let mut count = 0;

            while recipients.len() > 0 && count < all_givers.len() {
                let recipient = recipients.pop_front().unwrap();
                let mut recipient = givers.get(&recipient.name).unwrap().clone();

                if let Ok(()) = participant.validate_giving_to(&recipient) {
                    participant.giving_to = Some(recipient.name.clone());
                    recipient.receiving_from = Some(participant.name.clone());
                    recipient.excluding.push(participant.name.clone());
                    recipient.drawn = true;
                    givers.insert(participant.name.clone(), participant.clone());
                    givers.insert(recipient.name.clone(), recipient.clone());
                    break;
                } else {
                    recipient.drawn = true;
                    recipients.push_back(recipient);
                    count += 1;
                }
            }
            if count >= all_givers.len() {
                return self.assign_participants();
            }
        }

        Ok(givers.values().cloned().collect())

    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Participant {
    pub name: String,
    pub giving_to: Option<String>,
    pub receiving_from: Option<String>,
    pub excluding: Vec<String>,
    pub drawn: bool,
}

impl Participant {
    pub fn new(name: String) -> Participant {
        Participant {
            name,
            giving_to: None,
            receiving_from: None,
            excluding: Vec::new(),
            drawn: false,
        }
    }

    pub fn validate_giving_to(&self, recipient: &Participant) -> Result<(), SecretSatanError> {
        // Cannot give to yourself
        if self.name == recipient.name {
            return Err(SecretSatanError::ParticipantCannotGiveToThemself);
        }
        // Cannot give twice
        if self.giving_to.is_some() {
            return Err(SecretSatanError::ParticipantAlreadyGivingToSomeone);
        }
        // Cannot give to someone who is already receiving a gift
        if recipient.receiving_from.is_some() {
            return Err(SecretSatanError::ParticipantAlreadyReceivingFromSomeone);
        }
        // Cannot give to someone who is giving to you
        if self.receiving_from == Some(recipient.name.clone()) {
            return Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreReceivingFrom);
        }
        // Cannot give to someone you've excluded
        if self.excluding.contains(&recipient.name) {
            return Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreExcluding);
        }
        // Cannot give to someone who has already been drawn
        if recipient.drawn {
            return Err(SecretSatanError::ParticipantAlreadyDrawn);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecretSatanError {
    ParticipantAlreadyDrawn,
    ParticipantDoesNotExist,
    ParticipantAlreadyGivingToSomeone,
    ParticipantAlreadyReceivingFromSomeone,
    ParticipantCannotGiveToThemself,
    ParticipantCannotReceiveFromThemself,
    ParticipantCannotGiveToSomeoneTheyAreReceivingFrom,
    ParticipantCannotReceiveFromSomeoneTheyAreGivingTo,
    ParticipantCannotGiveToSomeoneTheyAreExcluding,
    ParticipantCannotReceiveFromSomeoneTheyAreExcluding,
}

/// A persistent storage hook that can be used to store data across application reloads.
#[allow(clippy::needless_return)]
pub fn use_persistent<T: Serialize + DeserializeOwned + Default + 'static>(
    // A unique key for the storage entry
    key: impl ToString,
    // A function that returns the initial value if the storage entry is empty
    init: impl FnOnce() -> T,
) -> UsePersistent<T> {
    // Use the use_signal hook to create a mutable state for the storage entry
    let state = use_signal(move || {
        // This closure will run when the hook is created
        let key = key.to_string();
        let value = LocalStorage::get(key.as_str()).ok().unwrap_or_else(init);
        StorageEntry { key, value }
    });

    // Wrap the state in a new struct with a custom API
    UsePersistent { inner: state }
}

struct StorageEntry<T> {
    key: String,
    value: T,
}

/// Storage that persists across application reloads
pub struct UsePersistent<T: 'static> {
    inner: Signal<StorageEntry<T>>,
}

impl<T> Clone for UsePersistent<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UsePersistent<T> {}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UsePersistent<T> {
    /// Returns a reference to the value
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    /// Sets the value
    pub fn set(&mut self, value: T) {
        let mut inner = self.inner.write();
        // Write the new value to local storage
        LocalStorage::set(inner.key.as_str(), &value).expect("unable to write to local storage");
        inner.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_satan_new() {
        let secret_satan = SecretSatan::new();
        assert_eq!(secret_satan.participants.len(), 0);
    }

    #[test]
    fn secret_satan_add_participant() {
        let mut secret_satan = SecretSatan::new();
        let participant = Participant::new("Alice".to_string());
        secret_satan.add_participant(participant.clone());
        assert_eq!(secret_satan.participants.len(), 1);
    }

    #[test]
    fn participant_new() {
        let participant = Participant::new("Alice".to_string());
        assert_eq!(participant.name, "Alice");
        assert_eq!(participant.giving_to, None);
        assert_eq!(participant.receiving_from, None);
        assert_eq!(participant.excluding.len(), 0);
        assert!(!participant.drawn);
    }

    #[test]
    fn participant_cannot_give_to_self() {
        let participant = Participant::new("Alice".to_string());
        let recipient = Participant::new("Alice".to_string());
        let result = participant.validate_giving_to(&recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToThemself));
    }

    #[test]
    fn participant_cannot_give_twice() {
        let mut participant = Participant::new("Alice".to_string());
        let recipient = Participant::new("Bob".to_string());
        participant.giving_to = Some(recipient.clone().name);
        let result = participant.validate_giving_to(&recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantAlreadyGivingToSomeone));
    }

    #[test]
    fn participant_cannot_give_to_someone_already_receiving() {
        let participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Bob".to_string());
        recipient.receiving_from = Some("Alice".to_string());
        let result = participant.validate_giving_to(&recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantAlreadyReceivingFromSomeone));
    }

    #[test]
    fn participant_cannot_give_to_someone_giving_to_them() {
        let mut participant = Participant::new("Alice".to_string());
        let recipient = Participant::new("Bob".to_string());
        participant.receiving_from = Some("Bob".to_string());
        let result = participant.validate_giving_to(&recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreReceivingFrom));
    }

    #[test]
    fn participant_cannot_give_to_someone_they_have_excluded() {
        let mut participant = Participant::new("Alice".to_string());
        let recipient = Participant::new("Bob".to_string());
        participant.excluding.push("Bob".to_string());
        let result = participant.validate_giving_to(&recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreExcluding));
    }

    #[test]
    fn participant_cannot_give_to_someone_already_drawn() {
        let participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Bob".to_string());
        recipient.drawn = true;
        let result = participant.validate_giving_to(&recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantAlreadyDrawn));
    }

    #[test]
    fn three_participants_can_give_correctly() {
        let alice = Participant::new("Alice".to_string());
        let bob = Participant::new("Bob".to_string());
        let charlie = Participant::new("Charlie".to_string());

        let session = SecretSatan {
            participants: vec![alice.clone(), bob.clone(), charlie.clone()],
        };
        let result = session.assign_participants();
        if let Err(e) = result {
            panic!("Error: {:?}", e);
        }
        let givers = result.ok().unwrap();
        assert_eq!(givers.len(), 3);
        assert_ne!(givers[0].giving_to, None);
        assert_ne!(givers[0].giving_to, Some(givers[0].clone().name));
        assert_ne!(givers[1].giving_to, None);
        assert_ne!(givers[1].giving_to, Some(givers[1].clone().name));
        assert_ne!(givers[2].giving_to, None);
        assert_ne!(givers[2].giving_to, Some(givers[2].clone().name));
    }

    #[test]
    fn five_participants_can_give_correctly() {
        let alice = Participant::new("Alice".to_string());
        let bob = Participant::new("Bob".to_string());
        let charlie = Participant::new("Charlie".to_string());
        let david = Participant::new("David".to_string());
        let eve = Participant::new("Eve".to_string());

        let session = SecretSatan {
            participants: vec![alice.clone(), bob.clone(), charlie.clone(), david.clone(), eve.clone()],
        };
        let result = session.assign_participants();
        if let Err(e) = result {
            panic!("Error: {:?}", e);
        }
        let givers = result.ok().unwrap();
        assert_eq!(givers.len(), 5);
        assert_ne!(givers[0].giving_to, None);
        assert_ne!(givers[0].giving_to, Some(givers[0].clone().name));
        assert_ne!(givers[1].giving_to, None);
        assert_ne!(givers[1].giving_to, Some(givers[1].clone().name));
        assert_ne!(givers[2].giving_to, None);
        assert_ne!(givers[2].giving_to, Some(givers[2].clone().name));
        assert_ne!(givers[3].giving_to, None);
        assert_ne!(givers[3].giving_to, Some(givers[3].clone().name));
        assert_ne!(givers[4].giving_to, None);
        assert_ne!(givers[4].giving_to, Some(givers[4].clone().name));
    }

    #[test]
    fn exclusions_are_accounted_for() {
        let mut alice = Participant::new("Alice".to_string());
        alice.excluding.push("Bob".to_string());
        let bob = Participant::new("Bob".to_string());
        let charlie = Participant::new("Charlie".to_string());

        let session = SecretSatan {
            participants: vec![alice.clone(), bob.clone(), charlie.clone()],
        };

        let result = session.assign_participants();
        if let Err(e) = result {
            panic!("Error: {:?}", e);
        }
        let givers = result.ok().unwrap();
        assert_eq!(givers.len(), 3);
        assert_ne!(givers[0].giving_to, None);
        assert_ne!(givers[0].giving_to, Some(givers[0].clone().name));
        assert_ne!(givers[0].giving_to, Some(givers[0].excluding.first().unwrap().clone()));
        assert_ne!(givers[1].giving_to, None);
        assert_ne!(givers[1].giving_to, Some(givers[1].clone().name));
        assert_ne!(givers[2].giving_to, None);
        assert_ne!(givers[2].giving_to, Some(givers[2].clone().name));
    }
}