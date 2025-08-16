use crate::auxiliary::io::Io;
use crate::bus::Bus;
use crate::cpu::interrupts::InterruptType;
use crate::ppu::fetcher::PixelFetcher;
use crate::ppu::lcd::{LcdStatSrc, PpuMode};
use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

pub const LINES_PER_FRAME: usize = 154;
pub const TICKS_PER_LINE: usize = 456;
pub const LCD_Y_RES: u8 = 144;
pub const LCD_X_RES: u8 = 160;
pub const TARGET_FPS_F: f64 = 59.7;
pub const TARGET_FRAME_TIME_MILLIS: u64 = FRAME_DURATION.as_millis() as u64;
pub const PPU_PIXELS_COUNT: usize = LCD_Y_RES as usize * LCD_X_RES as usize;
pub const PPU_BYTES_PER_PIXEL: usize = 2;
pub const PPU_BUFFER_LEN: usize = PPU_PIXELS_COUNT * PPU_BYTES_PER_PIXEL;
pub const PPU_PITCH: usize = PPU_BYTES_PER_PIXEL * LCD_X_RES as usize;

pub const FRAME_DURATION: Duration = Duration::from_nanos(16_743_000); // ~59.7 fps

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ppu {
    pub current_frame: usize,
    pub pipeline: PixelFetcher,
    line_ticks: usize,
    fps: Option<Fps>,
}

impl Ppu {
    pub fn reset(&mut self) {
        self.line_ticks = 0;
        self.current_frame = 0;
        self.pipeline.reset();
    }

    pub fn toggle_fps(&mut self, enable: bool) {
        if enable {
            self.fps = Some(Fps::default());
        } else {
            self.fps = None;
        }
    }

    pub fn get_fps(&mut self) -> Option<(&str, bool)> {
        self.fps.as_mut().map(|x| x.take())
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        self.line_ticks += 1;

        match bus.io.lcd.status.ppu_mode() {
            PpuMode::Oam => self.mode_oam(bus),
            PpuMode::Transfer => self.mode_transfer(bus),
            PpuMode::HBlank => self.mode_hblank(&mut bus.io),
            PpuMode::VBlank => self.mode_vblank(&mut bus.io),
        }
    }

    pub fn mode_oam(&mut self, bus: &mut Bus) {
        if self.line_ticks >= 80 {
            bus.io.lcd.status.set_ppu_mode(PpuMode::Transfer);
            self.pipeline.reset();
        }

        // todo:
        // GB fetches sprites progressively during the first 80 ticks of the scanline, not instantly
        //if self.line_ticks % 2 == 0 && self.line_ticks < 80 {
        //    self.pipeline.sprite_fetcher.load_next_sprite(bus);
        //}
        if self.line_ticks == 1 {
            // read oam on the first tick only
            self.pipeline.sprite_fetcher.load_line_sprites(bus);
        }
    }

    fn mode_transfer(&mut self, bus: &mut Bus) {
        self.pipeline.process(bus, self.line_ticks);

        if self.pipeline.is_full() {
            self.pipeline.clear();
            bus.io.lcd.status.set_ppu_mode(PpuMode::HBlank);

            if bus.io.lcd.status.is_stat_interrupt(LcdStatSrc::HBlank) {
                bus.io.interrupts.request_interrupt(InterruptType::LCDStat);
            }
        }
    }

    fn mode_vblank(&mut self, io: &mut Io) {
        if self.line_ticks >= TICKS_PER_LINE {
            io.lcd.increment_ly(&mut io.interrupts);

            if io.lcd.ly as usize >= LINES_PER_FRAME {
                io.lcd.status.set_ppu_mode(PpuMode::Oam);
                io.lcd.reset_ly(&mut io.interrupts);
            }

            self.line_ticks = 0;
        }
    }

    fn mode_hblank(&mut self, io: &mut Io) {
        if self.line_ticks >= TICKS_PER_LINE {
            io.lcd.increment_ly(&mut io.interrupts);

            if io.lcd.ly >= LCD_Y_RES {
                io.lcd.status.set_ppu_mode(PpuMode::VBlank);
                io.interrupts.request_interrupt(InterruptType::VBlank);

                if io.lcd.status.is_stat_interrupt(LcdStatSrc::VBlank) {
                    io.interrupts.request_interrupt(InterruptType::LCDStat);
                }

                self.current_frame += 1;

                if let Some(fps) = self.fps.as_mut() {
                    fps.update()
                }
            } else {
                io.lcd.status.set_ppu_mode(PpuMode::Oam);
            }

            self.line_ticks = 0;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fps {
    #[serde(with = "crate::instant_serde")]
    timer: Instant,
    prev_frame_time: Duration,
    last_fps_update: Duration,
    frame_accum: f32,
    frame_count: u32,
    fps: f32,
    fps_str: ArrayString<10>,
    updated: bool,
}

impl Default for Fps {
    fn default() -> Self {
        Self {
            timer: Instant::now(),
            prev_frame_time: Duration::ZERO,
            last_fps_update: Duration::ZERO,
            frame_accum: 0.0,
            frame_count: 0,
            fps: 0.0,
            fps_str: ArrayString::<10>::new(),
            updated: false,
        }
    }
}

use std::fmt::Write;

impl Fps {
    pub fn update(&mut self) {
        let now = self.timer.elapsed();
        let frame_time = (now - self.prev_frame_time).as_secs_f32();
        self.prev_frame_time = now;
        self.frame_accum += frame_time;
        self.frame_count += 1;

        if (now - self.last_fps_update).as_secs_f32() >= 1.0 {
            let new_fps = if self.frame_accum > 0.0 {
                self.frame_count as f32 / self.frame_accum
            } else {
                0.0
            };

            self.updated = (new_fps - self.fps).abs() > f32::EPSILON;

            if self.updated {
                self.fps = new_fps;
                self.fps_str.clear();
                write!(&mut self.fps_str, "{:.2}", self.fps).unwrap();
            }

            self.last_fps_update = now;
            self.frame_count = 0;
            self.frame_accum = 0.0;
        }
    }

    pub fn take(&mut self) -> (&str, bool) {
        let updated = self.updated;
        self.updated = false;

        (&self.fps_str, updated)
    }
}
