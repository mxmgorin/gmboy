use crate::auxiliary::io::Io;
use crate::core::ppu::oam::OamRam;
use crate::cpu::interrupts::InterruptType;
use crate::ppu::lcd::{Lcd, LcdMode, LcdStatSrc};
use crate::ppu::vram::VideoRam;
use std::thread;
use std::time::{Duration, Instant};

pub const LINES_PER_FRAME: usize = 154;
pub const TICKS_PER_LINE: usize = 456;
pub const Y_RES: usize = 144;
pub const X_RES: usize = 160;
pub const TARGET_FRAME_TIME: usize = 1000 / 60;

#[derive(Debug, Clone)]
pub struct Ppu {
    pub current_frame: usize,
    pub line_ticks: usize,
    pub prev_frame_time: usize,
    pub start_timer: usize,
    pub frame_count: usize,
    pub instant: Instant,

    pub video_buffer: Vec<u32>,
    pub video_ram: VideoRam,
    oam_ram: OamRam,
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Self {
            current_frame: 0,
            line_ticks: 0,
            prev_frame_time: 0,
            start_timer: 0,
            frame_count: 0,
            instant: Instant::now(),
            video_buffer: vec![0; Y_RES * X_RES],
            video_ram: VideoRam::new(),
            oam_ram: OamRam::new(),
        }
    }

    pub fn tick(&mut self, io: &mut Io) {
        self.line_ticks += 1;

        match io.lcd.status.mode() {
            LcdMode::Oam => self.mode_oam(&mut io.lcd),
            LcdMode::Xfer => self.mode_xfer(&mut io.lcd),
            LcdMode::HBlank => self.mode_hblank(io),
            LcdMode::VBlank => self.mode_vblank(io),
        }
    }

    pub fn vram_read(&self, addr: u16) -> u8 {
        self.video_ram.read(addr)
    }

    pub fn vram_write(&mut self, addr: u16, value: u8) {
        self.video_ram.write(addr, value);
    }

    pub fn oam_read(&self, addr: u16) -> u8 {
        self.oam_ram.read_byte(addr)
    }

    pub fn oam_write(&mut self, addr: u16, value: u8) {
        self.oam_ram.write_byte(addr, value);
    }

    pub fn mode_oam(&mut self, lcd: &mut Lcd) {
        if self.line_ticks >= 80 {
            lcd.status.mode_set(LcdMode::Xfer);
        }
    }

    fn mode_xfer(&mut self, lcd: &mut Lcd) {
        if self.line_ticks >= 80 + 172 {
            lcd.status.mode_set(LcdMode::HBlank);
        }
    }

    fn mode_vblank(&mut self, io: &mut Io) {
        if self.line_ticks >= TICKS_PER_LINE {
            io.lcd.increment_ly(&mut io.interrupts);

            if io.lcd.ly as usize >= LINES_PER_FRAME {
                io.lcd.status.mode_set(LcdMode::Oam);
                io.lcd.ly = 0;
            }

            self.line_ticks = 0;
        }
    }

    fn mode_hblank(&mut self, io: &mut Io) {
        if self.line_ticks >= TICKS_PER_LINE {
            io.lcd.increment_ly(&mut io.interrupts);

            if io.lcd.ly as usize >= Y_RES {
                io.lcd.status.mode_set(LcdMode::VBlank);
                io.interrupts.request_interrupt(InterruptType::VBlank);

                if io.lcd.status.stat_int(LcdStatSrc::HBlank) {
                    io.interrupts.request_interrupt(InterruptType::LCDStat);
                }

                self.current_frame += 1;

                // calc FPS
                let end = self.instant.elapsed().as_millis() as usize;
                println!("HBLANK TIME: {}ms", end);
                let frame_time = end - self.prev_frame_time;

                if frame_time < TARGET_FRAME_TIME {
                    thread::sleep(Duration::from_millis(
                        (TARGET_FRAME_TIME - frame_time) as u64,
                    ));
                }

                if end - self.start_timer >= 1000 {
                    println!("FPS: {}", self.frame_count);
                    self.frame_count = 0;
                    self.start_timer = 0;
                }

                self.frame_count += 1;
                self.prev_frame_time = self.instant.elapsed().as_millis() as usize;
            } else {
                io.lcd.status.mode_set(LcdMode::Oam);
            }
            
            self.line_ticks = 0;
        }
    }
}
