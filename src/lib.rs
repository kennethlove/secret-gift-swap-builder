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

    pub fn assign_participants(mut self) -> Result<Vec<Participant>, SecretSatanError> {
        let mut all_givers = self.participants.clone();
        let mut all_receivers = self.participants.clone();
        let mut output: Vec<Participant> = Vec::new();

        for mut giver in all_givers.clone() {
            let mut possible_receivers = all_receivers.clone();
            possible_receivers.retain(|receiver| {
                receiver.name != giver.name
            });

            for mut receiver in possible_receivers {
                match giver.gives_to(&mut receiver) {
                    Ok((giver, receiver)) => {
                        // Update giver and receiver in participants
                        let giver_index = self.participants.iter().position(|p| p.name == giver.name).unwrap();
                        let receiver_index = self.participants.iter().position(|p| p.name == receiver.name).unwrap();
                        all_givers[giver_index] = giver.clone();
                        all_givers[receiver_index] = receiver.clone();
                        all_receivers[giver_index] = giver.clone();
                        all_receivers[receiver_index] = receiver.clone();
                        output.push(giver.clone());
                        println!("{:?} gives to {:?}", giver, receiver);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        continue
                    }
                }
            }
        }

        Ok(all_givers)

    }
}

#[derive(Debug, Clone, Default, PartialEq)]
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

    pub fn default() -> Participant {
        Participant {
            name: String::new(),
            giving_to: None,
            receiving_from: None,
            excluding: Vec::new(),
            drawn: false,
        }
    }

    pub fn gives_to(&mut self, recipient: &mut Participant) -> Result<(Participant, Participant), SecretSatanError> {
        self.validate_giving_to(recipient)?;
        self.giving_to = Some(recipient.name.clone());
        recipient.receiving_from = Some(self.name.clone());
        recipient.excluding.push(self.name.clone());
        recipient.drawn = true;
        Ok((self.clone(), recipient.clone()))
    }

    fn validate_giving_to(&self, recipient: &mut Participant) -> Result<(), SecretSatanError> {
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
        // Cannot give to someone who has excluded you
        if recipient.excluding.contains(&self.name) {
            return Err(SecretSatanError::ParticipantCannotReceiveFromSomeoneTheyAreExcluding);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_satan_new() {
        let secret_satan = SecretSatan::new();
        assert_eq!(secret_satan.participants.len(), 0);
    }

    #[test]
    fn secret_satan_default() {
        let secret_satan = SecretSatan::default();
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
        assert_eq!(participant.drawn, false);
    }

    #[test]
    fn participant_default() {
        let participant = Participant::default();
        assert_eq!(participant.name, "");
        assert_eq!(participant.giving_to, None);
        assert_eq!(participant.receiving_from, None);
        assert_eq!(participant.excluding.len(), 0);
        assert_eq!(participant.drawn, false);
    }

    #[test]
    fn participant_cannot_give_to_self() {
        let mut participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Alice".to_string());
        let result = participant.gives_to(&mut recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToThemself));
    }

    #[test]
    fn participant_cannot_give_twice() {
        let mut participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Bob".to_string());
        participant.gives_to(&mut recipient).unwrap();
        let result = participant.gives_to(&mut recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantAlreadyGivingToSomeone));
    }

    #[test]
    fn participant_cannot_give_to_someone_already_receiving() {
        let mut participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Bob".to_string());
        recipient.receiving_from = Some("Alice".to_string());
        let result = participant.gives_to(&mut recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantAlreadyReceivingFromSomeone));
    }

    #[test]
    fn participant_cannot_give_to_someone_giving_to_them() {
        let mut participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Bob".to_string());
        participant.receiving_from = Some("Bob".to_string());
        let result = participant.gives_to(&mut recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreReceivingFrom));
    }

    #[test]
    fn participant_cannot_give_to_someone_they_have_excluded() {
        let mut participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Bob".to_string());
        participant.excluding.push("Bob".to_string());
        let result = participant.gives_to(&mut recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotGiveToSomeoneTheyAreExcluding));
    }

    #[test]
    fn participant_cannot_give_to_someone_who_has_excluded_them() {
        let mut participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Bob".to_string());
        recipient.excluding.push("Alice".to_string());
        let result = participant.gives_to(&mut recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantCannotReceiveFromSomeoneTheyAreExcluding));
    }

    #[test]
    fn participant_cannot_give_to_someone_already_drawn() {
        let mut participant = Participant::new("Alice".to_string());
        let mut recipient = Participant::new("Bob".to_string());
        recipient.drawn = true;
        let result = participant.gives_to(&mut recipient);
        assert_eq!(result, Err(SecretSatanError::ParticipantAlreadyDrawn));
    }

    #[test]
    fn three_participants_can_give_correctly() {
        let alice = Participant::new("Alice".to_string());
        let bob = Participant::new("Bob".to_string());
        let charlie = Participant::new("Charlie".to_string());

        let mut session = SecretSatan {
            participants: vec![alice.clone(), bob.clone(), charlie.clone()],
        };
        let result = session.assign_participants();
        if let Err(e) = result {
            panic!("Error: {:?}", e);
        }
        let givers = result.ok().unwrap();
        assert_eq!(givers[0].giving_to, Some("Bob".to_string()));
        assert_eq!(givers[1].giving_to, Some("Charlie".to_string()));
        assert_eq!(givers[2].giving_to, Some("Alice".to_string()));
    }
}