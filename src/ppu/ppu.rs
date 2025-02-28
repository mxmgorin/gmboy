use crate::auxiliary::io::Io;
use crate::bus::Bus;
use crate::cpu::interrupts::InterruptType;
use crate::ppu::lcd::{PpuMode, LcdStatSrc};
use crate::ppu::pipeline::Pipeline;
use std::time::{Duration, Instant};
use crate::auxiliary::clock::spin_wait;

pub const LINES_PER_FRAME: usize = 154;
pub const TICKS_PER_LINE: usize = 456;
pub const LCD_Y_RES: u8 = 144;
pub const LCD_X_RES: u8 = 160;
pub const TARGET_FPS_F: f64 = 60.0;
pub const TARGET_FRAME_TIME_MILLIS: u64 = 1000 / 60;
pub const LCD_PIXELS_COUNT: usize = LCD_Y_RES as usize * LCD_X_RES as usize;

#[derive(Debug, Clone)]
pub struct Ppu {
    pub current_frame: usize,
    pub line_ticks: usize,
    pub prev_frame_duration: Duration,
    pub frame_start_duration: Duration,
    pub last_frame_duration: Duration,
    pub target_frame_duration: Duration,
    pub frame_count: usize,
    pub fps: usize,
    pub timer: Instant,
    pub pipeline: Pipeline,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            current_frame: 0,
            line_ticks: 0,
            prev_frame_duration: Default::default(),
            frame_start_duration: Default::default(),
            last_frame_duration: Default::default(),
            target_frame_duration: Default::default(), // no frame limit
            frame_count: 0,
            fps: 0,
            timer: Instant::now(),
            pipeline: Default::default(),
        }
    }
}

impl Ppu {
    pub fn with_fps_limit(fps: f64) -> Ppu {
        Self {
            current_frame: 0,
            pipeline: Pipeline::default(),
            prev_frame_duration: Duration::new(0, 0),
            frame_start_duration: Duration::new(0, 0),
            last_frame_duration: Default::default(),
            target_frame_duration: Duration::from_secs_f64(1.0 / fps),
            frame_count: 0,
            fps: 0,
            timer: Instant::now(),
            line_ticks: 0,
        }
    }

    pub fn set_fps_limit(&mut self, fps: f64) {
        self.target_frame_duration = Duration::from_secs_f64(1.0 / fps);
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

        if self.pipeline.pushed_x >= LCD_X_RES {
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
                self.calc_fps();
                self.limit();
                self.prev_frame_duration = self.timer.elapsed();
            } else {
                io.lcd.status.set_ppu_mode(PpuMode::Oam);
            }

            self.line_ticks = 0;
        }
    }

    pub fn calc_fps(&mut self) {
        let current_duration = self.timer.elapsed();
        self.last_frame_duration = current_duration - self.prev_frame_duration;

        if (current_duration - self.frame_start_duration).as_millis() >= 1000 {
            self.fps = self.frame_count;
            self.frame_start_duration = current_duration;
            self.frame_count = 0;
        }

        self.frame_count += 1;
    }

    pub fn limit(&self) {
        if self.last_frame_duration < self.target_frame_duration {
            spin_wait(self.target_frame_duration - self.last_frame_duration);
        }
    }
}


