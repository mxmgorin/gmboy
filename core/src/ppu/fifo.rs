use crate::ppu::lcd::PixelColor;

const BUFFER_SIZE: usize = MAX_FIFO_SIZE * 2;
const MAX_FIFO_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct PixelFifo {
    buffer: [PixelColor; BUFFER_SIZE],
    head: usize,
    tail: usize,
    size: usize,
}

impl Default for PixelFifo {
    fn default() -> Self {
        Self {
            buffer: [PixelColor::default(); BUFFER_SIZE],
            head: 0,
            tail: 0,
            size: 0,
        }
    }
}

impl PixelFifo {
    pub fn push(&mut self, pixel: PixelColor) {
        // SAFETY:
        // - we change tail only here and don't give any mut reference
        unsafe {
            *self.buffer.get_unchecked_mut(self.tail) = pixel;
        }

        self.tail = (self.tail + 1) % BUFFER_SIZE;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<PixelColor> {
        if self.size > MAX_FIFO_SIZE {
            // SAFETY:
            // - we change head only here and don't give any mut reference
            // - buffer size is bigger than `MAX_FIFO_SIZE`
            let pixel = unsafe { *self.buffer.get_unchecked(self.head) };
            self.head = (self.head + 1) % BUFFER_SIZE;
            self.size -= 1;

            return Some(pixel);
        }

        None
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }

    pub fn is_full(&self) -> bool {
        self.size > MAX_FIFO_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ppu::lcd::PixelColor;

    fn create_pixel(v: u32) -> PixelColor {
        PixelColor::new(v as u8, 0, 0)
    }

    #[test]
    fn test_push_increases_size() {
        let mut fifo = PixelFifo::default();
        assert_eq!(fifo.size, 0);
        fifo.push(create_pixel(1));
        assert_eq!(fifo.size, 1);
    }

    #[test]
    fn test_is_full_behavior() {
        let mut fifo = PixelFifo::default();

        // Fill until just before full
        for i in 0..MAX_FIFO_SIZE {
            fifo.push(create_pixel(i as u32));
        }
        assert!(
            !fifo.is_full(),
            "Should not be full when size == MAX_FIFO_SIZE"
        );

        // Push one more
        fifo.push(create_pixel(255));
        assert!(fifo.is_full(), "Should be full when size > MAX_FIFO_SIZE");
    }

    #[test]
    fn test_pop_returns_none_when_not_full() {
        let mut fifo = PixelFifo::default();
        fifo.push(create_pixel(1));
        // size <= MAX_FIFO_SIZE, so pop should return None
        assert!(fifo.pop().is_none());
    }

    #[test]
    fn test_pop_returns_pixels_in_fifo_order() {
        let mut fifo = PixelFifo::default();

        // Fill enough to exceed MAX_FIFO_SIZE
        for i in 0..(MAX_FIFO_SIZE + 2) {
            fifo.push(create_pixel(i as u32));
        }

        // Now pop should return the first pushed pixels
        let p = fifo.pop();
        assert!(p.is_some());
        assert_eq!(p.unwrap(), create_pixel(0));

        let p2 = fifo.pop();
        assert!(p2.is_some());
        assert_eq!(p2.unwrap(), create_pixel(1));
    }

    #[test]
    fn test_clear_resets_state() {
        let mut fifo = PixelFifo::default();
        for i in 0..(MAX_FIFO_SIZE + 5) {
            fifo.push(create_pixel(i as u32));
        }
        fifo.clear();
        assert_eq!(fifo.size, 0);
        assert!(!fifo.is_full());
        assert!(fifo.pop().is_none());
    }

    #[test]
    fn test_wrap_around_tail() {
        let mut fifo = PixelFifo::default();

        // Fill to exceed buffer size to test wrapping
        for i in 0..(BUFFER_SIZE + 1) {
            fifo.push(create_pixel(i as u32));
        }

        // Popping after wrap should still work correctly
        while fifo.pop().is_some() {}
        assert!(fifo.pop().is_none());
    }
}
