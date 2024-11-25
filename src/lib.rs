use std::collections::VecDeque;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecretSatan {
    pub participants: Vec<Participant>,
}

impl SecretSatan {
    pub fn new() -> SecretSatan {
        SecretSatan {
            participants: Vec::new(),
        }
    }

    pub fn default() -> SecretSatan {
        SecretSatan {
            participants: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }

    pub fn assign_participants(&mut self) -> Result<Vec<Participant>, SecretSatanError> {
        let mut participants = VecDeque::from(self.participants.clone());
        let mut giving_to = participants.clone();
        let mut receiving_from = participants.clone();

        let mut processed_participants: Vec<Participant> = Vec::new();

        for mut participant in participants.iter_mut() {
            println!("Finding match for {}", participant.name);
            let mut giving_to_participant = giving_to.pop_front().unwrap();
            let mut receiving_from_participant = receiving_from.pop_front().unwrap();

            while giving_to_participant.name == participant.name &&
                !participant.excluding.contains(&giving_to_participant.name) &&
                !giving_to_participant.excluding.contains(&participant.name)
            {
                println!("{} cannot give to {}", &participant.name, &giving_to_participant.name);
                giving_to.push_back(giving_to_participant.clone());
                giving_to_participant = giving_to.pop_front().unwrap();
            }

            println!("{} is giving to {}", participant.name, giving_to_participant.name);
            participant.giving_to = Some(giving_to_participant.name.clone());

            while receiving_from_participant.name == participant.name &&
                !participant.excluding.contains(&receiving_from_participant.name) &&
                !receiving_from_participant.excluding.contains(&participant.name)
            {
                println!("{} cannot give to {}", &participant.name, &receiving_from_participant.name);
                receiving_from.push_back(receiving_from_participant.clone());
                receiving_from_participant = receiving_from.pop_front().unwrap();
            }

            println!("{} is giving to {}", participant.name, giving_to_participant.name);
            participant.giving_to = Some(giving_to_participant.name.clone());
            println!("{} is receiving from {}", participant.name, receiving_from_participant.name);
            receiving_from_participant.giving_to = Some(participant.name.clone());

            processed_participants.push(participant.clone());
        }

        Ok(processed_participants)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Participant {
    pub name: String,
    pub giving_to: Option<String>,
    pub receiving_from: Option<String>,
    pub excluding: Vec<String>,
}

impl Participant {
    pub fn new(name: String) -> Participant {
        Participant {
            name,
            giving_to: None,
            receiving_from: None,
            excluding: Vec::new(),
        }
    }

    pub fn default() -> Participant {
        Participant {
            name: String::new(),
            giving_to: None,
            receiving_from: None,
            excluding: Vec::new(),
        }
    }

    pub fn validate_giving_to(&self, participant: &Participant) -> Result<(), SecretSatanError> {
        if self.name == participant.name {
            return Err(SecretSatanError::ParticipantCannotGiveToThemselves);
        }
        if self.giving_to.is_some() {
            return Err(SecretSatanError::ParticipantAlreadyGivingToSomeone);
        }
        if self.receiving_from.is_some() && self.receiving_from.as_ref().unwrap() == &participant.name {
            return Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreReceivingFrom);
        }
        if self.excluding.contains(&participant.name) {
            return Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreExcluding);
        }
        Ok(())
    }

    pub fn set_giving_to(&mut self, participant: Participant) {
        println!("{} is giving to {}", self.name, participant.name);
        self.giving_to = Some(participant.name.clone());
    }

    pub fn validate_receiving_from(&self, participant: &Participant) -> Result<(), SecretSatanError> {
        if self.name == participant.name {
            return Err(SecretSatanError::ParticipantCannotReceiveFromThemselves);
        }
        if self.receiving_from.is_some() {
            return Err(SecretSatanError::ParticipantAlreadyReceivingFromSomeone);
        }
        if self.giving_to.is_some() && self.giving_to.as_ref().unwrap() == &participant.name {
            return Err(SecretSatanError::ParticipantCannotReceiveFromSomeoneTheyAreGivingTo);
        }
        if self.excluding.contains(&participant.name) {
            return Err(SecretSatanError::ParticipantCannotReceiveFromSomeoneTheyAreExcluding);
        }
        Ok(())
    }

    pub fn set_receiving_from(&mut self, participant: Participant) {
        println!("{} is receiving from {}", self.name, participant.name);
        self.receiving_from = Some(participant.name.clone());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecretSatanError {
    ParticipantAlreadyExists,
    ParticipantDoesNotExist,
    ParticipantAlreadyGivingToSomeone,
    ParticipantAlreadyReceivingFromSomeone,
    ParticipantCannotGiveToThemselves,
    ParticipantCannotReceiveFromThemselves,
    ParticipantCannotGiveToSomeoneTheyAreReceivingFrom,
    ParticipantCannotReceiveFromSomeoneTheyAreGivingTo,
    ParticipantCannotGiveToSomeoneTheyAreExcluding,
    ParticipantCannotReceiveFromSomeoneTheyAreExcluding,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_satan_new() {
        let secret_satan = SecretSatan::new();
        assert_eq!(secret_satan.participants.len(), 0);
    }

    #[test]
    fn test_secret_satan_default() {
        let secret_satan = SecretSatan::default();
        assert_eq!(secret_satan.participants.len(), 0);
    }

    #[test]
    fn test_secret_satan_add_participant() {
        let mut secret_satan = SecretSatan::new();
        let participant = Participant::new("Alice".to_string());
        secret_satan.add_participant(participant.clone());
        assert_eq!(secret_satan.participants.len(), 1);
    }

    #[test]
    fn test_participants_cannot_give_to_receivers() {
        let mut alice = Participant::new("Alice".to_string());
        let bob = Participant::new("Bob".to_string());
        alice.giving_to = Some(bob.name.clone());
        let result = alice.validate_receiving_from(&bob.clone());
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotReceiveFromSomeoneTheyAreGivingTo));
    }

    #[test]
    fn test_participants_cannot_give_to_themselves() {
        let mut alice = Participant::new("Alice".to_string());
        let result = alice.validate_giving_to(&alice.clone());
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToThemselves));
    }

    #[test]
    fn test_participant_cannot_give_to_excluded() {
        let mut alice = Participant::new("Alice".to_string());
        let bob = Participant::new("Bob".to_string());
        alice.excluding.push(bob.name.clone());
        let result = alice.validate_giving_to(&bob.clone());
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreExcluding));
    }

    #[test]
    fn test_participants_cannot_receive_from_givers() {
        let mut alice = Participant::new("Alice".to_string());
        let bob = Participant::new("Bob".to_string());
        alice.receiving_from = Some(bob.name.clone());
        let result = alice.validate_giving_to(&bob.clone());
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreReceivingFrom));
    }

    #[test]
    fn test_participants_cannot_receive_from_themselves() {
        let mut alice = Participant::new("Alice".to_string());
        let result = alice.validate_receiving_from(&alice.clone());
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotReceiveFromThemselves));
    }

    #[test]
    fn test_participant_cannot_receive_from_excluded() {
        let mut alice = Participant::new("Alice".to_string());
        let bob = Participant::new("Bob".to_string());
        alice.excluding.push(bob.name.clone());
        let result = alice.validate_receiving_from(&bob.clone());
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotReceiveFromSomeoneTheyAreExcluding));
    }

    #[test]
    fn test_participant_assignment() {
        let mut secret_satan = SecretSatan::new();
        let alice = Participant::new("Alice".to_string());
        let bob = Participant::new("Bob".to_string());
        let charlie = Participant::new("Charlie".to_string());
        secret_satan.add_participant(alice.clone());
        secret_satan.add_participant(bob.clone());
        secret_satan.add_participant(charlie.clone());
        let assignments = secret_satan.assign_participants().expect("failed to assign participants");

        assert_eq!(assignments[0].giving_to, Some(bob.clone().name));
        assert_eq!(assignments[0].receiving_from, Some(charlie.clone().name));

        assert_eq!(assignments[1].giving_to, Some(charlie.clone().name));
        assert_eq!(assignments[1].receiving_from, Some(alice.clone().name));

        assert_eq!(assignments[2].giving_to, Some(alice.clone().name));
        assert_eq!(assignments[2].receiving_from, Some(bob.clone().name));
    }
}