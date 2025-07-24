use crate::auxiliary::clock::Tickable;
use crate::auxiliary::io::Io;
use crate::bus::Bus;
use crate::cpu::interrupts::InterruptType;
use crate::ppu::fetcher::PixelFetcher;
use crate::ppu::lcd::{LcdStatSrc, PpuMode};
use std::time::{Duration, Instant};

pub const LINES_PER_FRAME: usize = 154;
pub const TICKS_PER_LINE: usize = 456;
pub const LCD_Y_RES: u8 = 144;
pub const LCD_X_RES: u8 = 160;
pub const TARGET_FPS_F: f64 = 59.7;
pub const TARGET_FRAME_TIME_MILLIS: u64 = FRAME_DURATION.as_millis() as u64;
pub const LCD_PIXELS_COUNT: usize = LCD_Y_RES as usize * LCD_X_RES as usize;
pub const FRAME_DURATION: Duration = Duration::from_nanos(16_743_000); // ~59.7 fps

impl Tickable for Ppu {
    fn tick(&mut self, bus: &mut Bus) {
        self.tick(bus);
    }
}

#[derive(Debug, Clone)]
pub struct Ppu {
    pub current_frame: usize,
    pub line_ticks: usize,
    pub prev_frame_duration: Duration,
    pub frame_start_duration: Duration,
    pub last_frame_duration: Duration,
    pub frame_count: usize,
    pub fps_counter: Option<FpsCounter>,
    pub timer: Instant,
    pub pipeline: PixelFetcher,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            current_frame: 0,
            line_ticks: 0,
            prev_frame_duration: Default::default(),
            frame_start_duration: Default::default(),
            last_frame_duration: Default::default(),
            frame_count: 0,
            timer: Instant::now(),
            pipeline: Default::default(),
            fps_counter: None,
        }
    }
}

impl Ppu {
    pub fn toggle_fps_counter(&mut self) {
        if self.fps_counter.is_some() {
            self.fps_counter = None;
        } else {
            self.fps_counter = Some(FpsCounter::default());
        }
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
                self.prev_frame_duration = self.timer.elapsed();

                if let Some(fps_counter) = self.fps_counter.as_mut() {
                    fps_counter.update()
                }
            } else {
                io.lcd.status.set_ppu_mode(PpuMode::Oam);
            }

            self.line_ticks = 0;
        }
    }
}

#[derive(Debug, Clone)]
pub struct FpsCounter {
    timer: Instant,
    prev_frame_time: Duration,
    last_fps_update: Duration,
    frame_accum: f32,
    frame_count: u32,
    fps: f32,
    fps_str: String,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            timer: Instant::now(),
            prev_frame_time: Duration::ZERO,
            last_fps_update: Duration::ZERO,
            frame_accum: 0.0,
            frame_count: 0,
            fps: 0.0,
            fps_str: "0.0".to_string(),
        }
    }
}

impl FpsCounter {
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

            self.fps_str = format!("{:.1}", self.fps);
            self.last_fps_update = now;
            self.frame_count = 0;
            self.frame_accum = 0.0;
        }
    }

    pub fn fps_str(&self) -> &str {
        &self.fps_str
    }
}
