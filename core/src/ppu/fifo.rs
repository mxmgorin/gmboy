use crate::ppu::tile::Pixel;

const BUFFER_SIZE: usize = MAX_FIFO_SIZE * 2;
const MAX_FIFO_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct PixelFifo {
    buffer: [Pixel; BUFFER_SIZE],
    head: usize,
    tail: usize,
    size: usize,
}

impl Default for PixelFifo {
    fn default() -> Self {
        Self {
            buffer: [Pixel::default(); BUFFER_SIZE],
            head: 0,
            tail: 0,
            size: 0,
        }
    }
}

impl PixelFifo {
    pub fn push(&mut self, pixel: Pixel) {
        self.buffer[self.tail] = pixel;
        self.tail = (self.tail + 1) % BUFFER_SIZE;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<Pixel> {
        if self.size > MAX_FIFO_SIZE {
            let pixel = self.buffer[self.head];
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
