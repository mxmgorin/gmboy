use std::time::{Duration, Instant};

struct Notification {
    pub text: String,
    pub created_at: Instant,
}

pub struct Notifications {
    items: Vec<Notification>,
    duration: Duration,
    buffer: Vec<*const str>, // internal buffer for active popup messages
}

impl Notifications {
    pub fn new(duration: Duration) -> Self {
        Self {
            items: Vec::with_capacity(4),
            duration,
            buffer: Vec::with_capacity(4),
        }
    }

    pub fn add(&mut self, text: impl Into<String>) {
        self.items.push(Notification {
            text: text.into(),
            created_at: Instant::now(),
        });
    }

    pub fn update_and_get(&mut self) -> &[&str] {
        if self.items.is_empty() {
            return &[];
        }

        let now = Instant::now();
        self.buffer.clear();

        let mut i = 0;
        while i < self.items.len() {
            if now.duration_since(self.items[i].created_at) < self.duration {
                // Store raw pointer to &str
                let ptr: *const str = self.items[i].text.as_str();
                self.buffer.push(ptr);
                i += 1;
            } else {
                self.items.swap_remove(i);
            }
        }

        // SAFETY:
        // all pointers in `self.refs` point to valid strings in `self.messages`
        // Convert &[ *const str ] -> &[ &str ] by transmuting each element
        unsafe {
            std::slice::from_raw_parts(self.buffer.as_ptr() as *const &str, self.buffer.len())
        }
    }
}
