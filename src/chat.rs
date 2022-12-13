use std::time::SystemTime;

pub struct ChatMessage {
    pub sender: String,
    pub message: String,
    pub timestamp: SystemTime,
}

pub struct Chat {
    participants: Vec<String>,
    history: Vec<ChatMessage>,
}

impl Chat {
    pub fn new() -> Chat {
        Chat {
            participants: vec![],
            history: vec![],
        }
    }

    pub fn add_message(&mut self, sender: String, message: String, timestamp: SystemTime) {
        let mut index = self.participants.iter().find(|&x| x == &sender);
        if !self.participants.contains(&sender) {
            self.participants.push(sender.to_owned());
        }

        self.history.push(ChatMessage {
            sender: sender,
            message,
            timestamp,
        });
    }

    pub fn get_participants(&self) -> &Vec<String> {
        &self.participants
    }

    pub fn get_history(&self) -> &Vec<ChatMessage> {
        &self.history
    }
}