use std::time::{Duration, Instant};

pub struct PopupMessage {
    pub text: String,
    pub created_at: Instant,
}

pub struct PopupManager {
    messages: Vec<PopupMessage>,
    duration: Duration,
}

impl PopupManager {
    pub fn new(duration: Duration) -> Self {
        Self {
            messages: Vec::new(),
            duration,
        }
    }

    pub fn show(&mut self, text: impl Into<String>) {
        self.messages.push(PopupMessage {
            text: text.into(),
            created_at: Instant::now(),
        });
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.messages
            .retain(|msg| now.duration_since(msg.created_at) < self.duration);
    }

    pub fn get(&self) -> &[PopupMessage] {
        &self.messages
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
