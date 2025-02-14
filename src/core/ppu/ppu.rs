use crate::auxiliary::io::Io;
use crate::bus::Bus;
use crate::cpu::interrupts::InterruptType;
use crate::ppu::lcd::{LcdMode, LcdStatSrc};
use crate::ppu::pipeline::{Pipeline, PipelineState};
use std::thread;
use std::time::{Duration, Instant};

pub const LINES_PER_FRAME: usize = 154;
pub const TICKS_PER_LINE: usize = 456;
pub const LCD_Y_RES: u8 = 144;
pub const LCD_X_RES: u8 = 160;
pub const TARGET_FRAME_TIME_MILLIS: u64 = 1000 / 60;
pub const TARGET_FRAME_DURATION: Duration = Duration::from_millis(TARGET_FRAME_TIME_MILLIS);

#[derive(Debug, Clone)]
pub struct Ppu {
    pub current_frame: usize,
    pub prev_frame_duration: Duration,
    pub start_duration: Duration,
    pub frame_count: usize,
    pub fps: usize,
    pub instant: Instant,
    pub pipeline: Pipeline,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            current_frame: 0,
            pipeline: Pipeline::default(),
            prev_frame_duration: Duration::new(0, 0),
            start_duration: Duration::new(0, 0),
            frame_count: 0,
            fps: 0,
            instant: Instant::now(),
        }
    }
}

impl Ppu {
    pub fn tick(&mut self, bus: &mut Bus) {
        self.pipeline.line_ticks += 1;

        match bus.io.lcd.status.mode() {
            LcdMode::Oam => self.mode_oam(bus),
            LcdMode::Transfer => self.mode_transfer(bus),
            LcdMode::HBlank => self.mode_hblank(&mut bus.io),
            LcdMode::VBlank => self.mode_vblank(&mut bus.io),
        }
    }

    pub fn mode_oam(&mut self, bus: &mut Bus) {
        if self.pipeline.line_ticks >= 80 {
            bus.io.lcd.status.mode_set(LcdMode::Transfer);
            self.pipeline.state = PipelineState::Tile;
            self.pipeline.line_x = 0;
            self.pipeline.fetch_x = 0;
            self.pipeline.pushed_x = 0;
            self.pipeline.fifo_x = 0;
        }

        if self.pipeline.line_ticks == 1 {
            // read oam on the first tick only
            self.pipeline.sprite_fetcher.load_line_sprites(bus);
        }
    }

    fn mode_transfer(&mut self, bus: &mut Bus) {
        self.pipeline.process(bus);

        if self.pipeline.pushed_x >= LCD_X_RES {
            self.pipeline.reset();
            bus.io.lcd.status.mode_set(LcdMode::HBlank);

            if bus.io.lcd.status.is_stat_interrupt(LcdStatSrc::HBlank) {
                bus.io.interrupts.request_interrupt(InterruptType::LCDStat);
            }
        }
    }

    fn mode_vblank(&mut self, io: &mut Io) {
        if self.pipeline.line_ticks >= TICKS_PER_LINE {
            io.lcd.increment_ly(&mut io.interrupts);

            if io.lcd.ly as usize >= LINES_PER_FRAME {
                io.lcd.status.mode_set(LcdMode::Oam);
                io.lcd.ly = 0;
            }

            self.pipeline.line_ticks = 0;
        }
    }

    fn mode_hblank(&mut self, io: &mut Io) {
        if self.pipeline.line_ticks >= TICKS_PER_LINE {
            io.lcd.increment_ly(&mut io.interrupts);

            if io.lcd.ly >= LCD_Y_RES {
                io.lcd.status.mode_set(LcdMode::VBlank);
                io.interrupts.request_interrupt(InterruptType::VBlank);

                if io.lcd.status.is_stat_interrupt(LcdStatSrc::HBlank) {
                    io.interrupts.request_interrupt(InterruptType::LCDStat);
                }

                self.current_frame += 1;
                self.calc_fps();
                self.prev_frame_duration = self.instant.elapsed();
            } else {
                io.lcd.status.mode_set(LcdMode::Oam);
            }

            self.pipeline.line_ticks = 0;
        }
    }

    pub fn calc_fps(&mut self) {
        let end = self.instant.elapsed();
        let frame_duration = end - self.prev_frame_duration;

        if frame_duration < TARGET_FRAME_DURATION {
            thread::sleep(TARGET_FRAME_DURATION - frame_duration);
        }

        if (end - self.start_duration).as_millis() >= 1000 {
            self.fps = self.frame_count;
            self.start_duration = end;
            self.frame_count = 0;
        }

        self.frame_count += 1;
    }
}
