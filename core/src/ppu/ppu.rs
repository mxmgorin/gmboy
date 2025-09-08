use crate::cpu::interrupts::{InterruptType, Interrupts};
use crate::ppu::fetcher::PixelFetcher;
use crate::ppu::framebuffer::FrameBuffer;
use crate::ppu::lcd::{Lcd, LcdStatSrc, PpuMode};
use crate::ppu::oam::OamRam;
use crate::ppu::vram::VideoRam;
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
    pub buffer: FrameBuffer,
    pub video_ram: VideoRam,
    pub oam_ram: OamRam,
    pub lcd: Lcd,
    pub current_frame: usize,
    line_ticks: usize,
    fps: Option<Fps>,
    fetcher: PixelFetcher,
}

impl Ppu {
    pub fn new(lcd: Lcd) -> Self {
        Self {
            lcd,
            ..Default::default()
        }
    }

    pub fn toggle_fps(&mut self, enable: bool) {
        if enable {
            self.fps = Some(Fps::default());
        } else {
            self.fps = None;
        }
    }

    #[inline(always)]
    pub fn get_fps(&mut self) -> Option<f32> {
        self.fps.as_mut().map(|x| x.get())
    }

    #[inline(always)]
    pub fn tick(&mut self, interrupts: &mut Interrupts) {
        self.line_ticks += 1;

        match self.lcd.status.get_ppu_mode() {
            PpuMode::HBlank => self.mode_hblank(interrupts),
            PpuMode::VBlank => self.mode_vblank(interrupts),
            PpuMode::Oam => self.mode_oam(),
            PpuMode::Transfer => self.mode_transfer(interrupts),
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.line_ticks = 0;
        self.current_frame = 0;
        self.buffer.reset();
        self.fetcher.reset();
    }

    #[inline(always)]
    fn mode_oam(&mut self) {
        if self.line_ticks >= 80 {
            self.fetcher
                .sprite_fetcher
                .scan_oam(&self.lcd, &self.oam_ram);
            self.set_mode_transfer();
        }
    }

    #[inline(always)]
    fn mode_transfer(&mut self, interrupts: &mut Interrupts) {
        let color = self
            .fetcher
            .fetch(&self.lcd, &self.video_ram, self.line_ticks);

        if let Some(color) = color {
            self.buffer.push(self.lcd.ly as usize, color);
        }

        if self.buffer.count() >= LCD_X_RES as usize {
            self.set_mode_hblank(interrupts);
        }
    }

    #[inline(always)]
    fn mode_vblank(&mut self, interrupts: &mut Interrupts) {
        if self.line_ticks >= TICKS_PER_LINE {
            self.lcd.increment_ly(interrupts);

            if self.lcd.ly as usize >= LINES_PER_FRAME {
                self.set_mode_oam(interrupts);
                self.lcd.reset_ly(interrupts);
            }

            self.line_ticks = 0;
        }
    }

    #[inline(always)]
    fn mode_hblank(&mut self, interrupts: &mut Interrupts) {
        if self.line_ticks >= TICKS_PER_LINE {
            self.lcd.increment_ly(interrupts);

            if self.lcd.ly >= LCD_Y_RES {
                self.set_mode_vblank(interrupts);
                self.current_frame += 1;

                if let Some(fps) = self.fps.as_mut() {
                    fps.update()
                }
            } else {
                self.set_mode_oam(interrupts);
            }

            self.line_ticks = 0;
        }
    }

    #[inline(always)]
    const fn set_mode_oam(&mut self, interrupts: &mut Interrupts) {
        self.lcd.status.set_ppu_mode(PpuMode::Oam);

        if self.lcd.status.is_stat_interrupt(LcdStatSrc::Oam) {
            interrupts.request_interrupt(InterruptType::LCDStat);
        }
    }

    #[inline(always)]
    const fn set_mode_transfer(&mut self) {
        self.buffer.reset();
        self.fetcher.reset();
        self.lcd.status.set_ppu_mode(PpuMode::Transfer);
    }

    #[inline(always)]
    const fn set_mode_hblank(&mut self, interrupts: &mut Interrupts) {
        self.lcd.status.set_ppu_mode(PpuMode::HBlank);

        // TODO: STAT mode=0 interrupt happens one cycle before the actual mode switch!
        if self.lcd.status.is_stat_interrupt(LcdStatSrc::HBlank) {
            interrupts.request_interrupt(InterruptType::LCDStat);
        }
    }

    #[inline(always)]
    const fn set_mode_vblank(&mut self, interrupts: &mut Interrupts) {
        self.lcd.status.set_ppu_mode(PpuMode::VBlank);
        interrupts.request_interrupt(InterruptType::VBlank);

        if self.lcd.status.is_stat_interrupt(LcdStatSrc::VBlank) {
            interrupts.request_interrupt(InterruptType::LCDStat);
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
        }
    }
}

impl Fps {
    #[inline(always)]
    pub fn update(&mut self) {
        let now = self.timer.elapsed();
        let frame_time = (now - self.prev_frame_time).as_secs_f32();
        self.prev_frame_time = now;
        self.frame_accum += frame_time;
        self.frame_count += 1;

        if (now - self.last_fps_update).as_secs_f32() >= 1.0 {
            self.fps = if self.frame_accum > 0.0 {
                self.frame_count as f32 / self.frame_accum
            } else {
                0.0
            };

            self.last_fps_update = now;
            self.frame_count = 0;
            self.frame_accum = 0.0;
        }
    }

    #[inline(always)]
    pub fn get(&mut self) -> f32 {
        self.fps
    }
}
