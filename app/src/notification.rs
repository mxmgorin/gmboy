use std::time::{Duration, Instant};

const MAX_CHARS: usize = 34;
const MAX_COUNT: usize = 4;

struct Notification {
    pub text: String,
    pub created_at: Instant,
}

pub struct Notifications {
    items: Vec<Notification>,
    duration: Duration,
    buffer: Vec<*const str>, // internal buffer for active popup messages
    updated: bool,
}

impl Notifications {
    pub fn new(duration: Duration) -> Self {
        Self {
            items: Vec::with_capacity(4),
            duration,
            buffer: Vec::with_capacity(4),
            updated: false,
        }
    }

    pub fn add(&mut self, text: impl Into<String>) {
        let text = text.into();
        let mut truncated = String::with_capacity(MAX_CHARS + 2);
        let mut chars = text.chars();

        for _ in 0..MAX_CHARS {
            if let Some(c) = chars.next() {
                truncated.push(c);
            } else {
                break;
            }
        }

        // If there are still more characters, append ".."
        if chars.next().is_some() {
            truncated.push_str("..");
        }

        let item = Notification {
            text: truncated,
            created_at: Instant::now(),
        };
        let len = self.items.len();
        if len >= MAX_COUNT {
            self.items[len - 1] = item;
        } else {
            self.items.push(item);
        }

        self.updated = true;
    }

    pub fn update_and_get(&mut self) -> (&[&str], bool) {
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
                self.updated = true;
            }
        }

        // SAFETY:
        // all pointers in `self.refs` point to valid strings in `self.messages`
        // Convert &[ *const str ] -> &[ &str ] by transmuting each element
        let result = unsafe {
            std::slice::from_raw_parts(self.buffer.as_ptr() as *const &str, self.buffer.len())
        };

        //let updated = prev_len != self.items.len();
        let updated = self.updated;
        self.updated = false;

        (result, updated)
    }
}
