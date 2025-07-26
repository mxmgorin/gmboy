use std::time::{Duration, Instant};

struct PopupMessage {
    pub text: String,
    pub created_at: Instant,
}

pub struct Popups {
    messages: Vec<PopupMessage>,
    duration: Duration,
    refs: Vec<*const str>, // internal buffer for active popup messages
}

impl Popups {
    pub fn new(duration: Duration) -> Self {
        Self {
            messages: Vec::with_capacity(4),
            duration,
            refs: Vec::with_capacity(4),
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

    pub fn update_and_get(&mut self) -> &[&str] {
        let now = Instant::now();
        self.refs.clear();

        let mut i = 0;
        while i < self.messages.len() {
            if now.duration_since(self.messages[i].created_at) < self.duration {
                // Store raw pointer to &str
                let ptr: *const str = self.messages[i].text.as_str();
                self.refs.push(ptr);
                i += 1;
            } else {
                self.messages.swap_remove(i);
            }
        }

        // SAFETY:
        // all pointers in `self.refs` point to valid strings in `self.messages`
        // Convert &[ *const str ] -> &[ &str ] by transmuting each element
        unsafe { std::slice::from_raw_parts(self.refs.as_ptr() as *const &str, self.refs.len()) }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
