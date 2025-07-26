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

    pub fn add(&mut self, text: impl Into<String>) {
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

    pub fn update_and_get(&mut self) -> Vec<&str> {
        let now = Instant::now();
        let mut i = 0;
        let mut refs = Vec::with_capacity(self.messages.len());

        while i < self.messages.len() {
            if now.duration_since(self.messages[i].created_at) < self.duration {
                // SAFETY:
                // we only take a reference to `self.messages[i].text` while
                // that element stays in place. No mutation invalidates this reference.
                let ptr = &self.messages[i].text as *const String;
                refs.push(unsafe { &*ptr }.as_str());
                i += 1;
            } else {
                self.messages.swap_remove(i);
            }
        }

        refs
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
