use crate::audio::AppAudio;
use crate::config::AppConfig;
use crate::input::InputHandler;
use crate::video::draw_text::FontSize;
use crate::video::game_window::GameWindow;
use crate::video::menu::AppMenu;
use crate::video::tiles_window::TileWindow;
use crate::Emu;
use core::emu::battery::BatterySave;
use core::emu::runtime::EmuRuntime;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCommand;
use core::emu::EmuCallback;
use core::into_pixel_colors;
use core::ppu::palette::LcdPalette;
use core::ppu::tile::PixelColor;
use sdl2::{Sdl, VideoSubsystem};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub enum AppCommand {
    TogglePause,
    Rewind,
    LoadFile(PathBuf),
    Restart,
    ChangeMode(RunMode),
    SaveState(SaveStateCommand, usize),
    PickFile,
    Quit,
    ChangeConfig(ChangeAppConfigCommand)
}

pub enum ChangeAppConfigCommand {
    Volume(f32),
    Scale(f32),
    TileWindow,
    Fullscreen,
    Fps,
    SpinDuration(i32),
    NextPalette,
    ToggleMute,
    NormalSpeed(f32),
    TurboSpeed(f32),
    SlowSpeed(f32),
    RewindSize(i32),
    RewindInterval(i32),
    SaveStateOnExit,
    AudioBufferSize(i32),
    MuteTurbo,
    MuteSlow,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AppState {
    Paused,
    Running,
    Quitting,
}

pub struct App {
    video_subsystem: VideoSubsystem,
    audio: AppAudio,
    window: GameWindow,
    palettes: Box<[LcdPalette]>,
    pub tile_window: Option<TileWindow>,
    pub state: AppState,
    pub config: AppConfig,
    pub menu: AppMenu,
}

impl EmuCallback for App {
    fn update_video(&mut self, buffer: &[u32], runtime: &EmuRuntime) {
        self.window.draw_buffer(buffer);

        if self.config.interface.show_fps {
            if let Some(fps) = &runtime.ppu.fps {
                self.window.draw_fps(
                    fps.display(),
                    FontSize::Normal,
                    runtime.bus.io.lcd.current_colors[0],
                );
            }
        }

        self.window.present();
    }

    fn update_audio(&mut self, output: &[f32], runtime: &EmuRuntime) {
        if self.config.audio.mute {
            return;
        }

        if self.config.audio.mute_turbo && runtime.mode == RunMode::Turbo {
            return;
        }

        if self.config.audio.mute_slow && runtime.mode == RunMode::Slow {
            return;
        }

        self.audio.queue(output);
    }
}

impl App {
    pub fn new(
        sdl: &mut Sdl,
        config: AppConfig,
        palettes: Box<[LcdPalette]>,
    ) -> Result<Self, String> {
        let video_subsystem = sdl.video()?;
        let mut game_window = GameWindow::new(config.interface.scale as u32, &video_subsystem)?;
        game_window.set_fullscreen(config.interface.is_fullscreen);

        let tile_window = if config.interface.tile_window {
            let (x, y) = game_window.get_position();
            let mut debug_window = TileWindow::new(&video_subsystem);
            debug_window.set_position(x + 640, y);

            Some(debug_window)
        } else {
            None
        };

        Ok(Self {
            video_subsystem,
            tile_window,
            audio: AppAudio::new(sdl, &config.audio),
            window: game_window,
            menu: AppMenu::new(config.last_cart_path.is_some()),
            state: AppState::Paused,
            palettes,
            config,
        })
    }

    pub fn toggle_tile_window(&mut self) {
        if self.tile_window.is_some() {
            self.tile_window = None;
            self.config.interface.tile_window = false;
        } else {
            self.tile_window = Some(TileWindow::new(&self.video_subsystem));
            self.config.interface.tile_window = true;
        }
    }

    /// Execution loop
    pub fn run(&mut self, emu: &mut Emu, input: &mut InputHandler) -> Result<(), String> {
        self.state = AppState::Paused;

        while self.state != AppState::Quitting {
            while self.state == AppState::Paused {
                self.paused(emu, input);
            }

            input.handle_events(self, emu);
            emu.run_frame(self)?;

            if let Some(tiles_window) = self.tile_window.as_mut() {
                tiles_window.draw_tiles(emu.runtime.bus.video_ram.iter_tiles());
            }
        }

        Ok(())
    }

    pub fn paused(&mut self, emu: &mut Emu, input: &mut InputHandler) {
        input.handle_events(self, emu);
        emu.runtime.clock.reset();
        let text_color = emu.runtime.bus.io.lcd.current_colors[0];
        let bg_color = emu.runtime.bus.io.lcd.current_colors[3];
        self.draw_menu(text_color, bg_color);

        thread::sleep(Duration::from_millis(100));
    }

    pub fn change_scale(&mut self, delta: f32) -> Result<(), String> {
        self.config.interface.scale += delta;
        self.window.set_scale(self.config.interface.scale as u32)?;

        println!("Current scale: {}", self.config.interface.scale);

        Ok(())
    }

    fn draw_menu(&mut self, text_color: PixelColor, bg_color: PixelColor) {
        let items = self.menu.get_items(&self.config);
        let items: Vec<&str> = items.iter().map(|s| s.as_str()).collect();

        self.draw_text(&items, text_color, bg_color, true);
    }

    fn draw_text(
        &mut self,
        lines: &[&str],
        text_color: PixelColor,
        bg_color: PixelColor,
        align_center: bool,
    ) {
        self.window
            .draw_text_lines(lines, FontSize::Small, text_color, bg_color, align_center);

        self.window.present();
    }

    pub fn next_palette(&mut self, emu: &mut Emu) {
        self.config.interface.selected_palette_idx = core::move_next_wrapped(
            self.config.interface.selected_palette_idx,
            self.palettes.len() - 1,
        );
        let palette = &self.palettes[self.config.interface.selected_palette_idx];
        emu.runtime
            .bus
            .io
            .lcd
            .set_pallet(into_pixel_colors(&palette.hex_colors));

        println!("Current palette: {}", palette.name);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.config.interface.is_fullscreen = !self.config.interface.is_fullscreen;
        self.window
            .set_fullscreen(self.config.interface.is_fullscreen);
    }

    pub fn handle_save_state(&mut self, emu: &mut Emu, event: SaveStateCommand, index: usize) {
        let name = self.config.get_last_file_stem().unwrap();

        match event {
            SaveStateCommand::Create => {
                let save_state = emu.create_save_state();

                if let Err(err) = save_state.save_file(&name, index) {
                    eprintln!("Failed save_state: {err}");
                    return;
                }

                println!("Saved save state: {index}");
                self.state = AppState::Running;
            }
            SaveStateCommand::Load => {
                let save_state = core::emu::runtime::EmuSaveState::load_file(&name, index);

                let Ok(save_state) = save_state else {
                    eprintln!("Failed load save_state: {}", save_state.unwrap_err());
                    return;
                };

                emu.load_save_state(save_state);
                println!("Loaded save state: {index}");
                self.state = AppState::Running;
            }
        }
    }

    pub fn save_files(&mut self, emu: &mut Emu) -> Result<(), String> {
        // save config
        self.config.set_emu_config(emu.config.clone());
        if let Err(err) = self.config.save_file().map_err(|e| e.to_string()) {
            eprint!("Failed config.save: {err}");
        }

        // save sram for battery emulation
        let name = self.config.get_last_file_stem().unwrap();

        if let Some(bytes) = emu.runtime.bus.cart.dump_ram() {
            let battery = BatterySave::from_bytes(bytes)
                .save_file(&name)
                .map_err(|e| e.to_string());

            if let Err(err) = battery {
                eprint!("Failed BatterySave: {err}");
            };
        }

        // save state on exit
        if self.config.save_state_on_exit {
            if let Err(err) = emu.create_save_state().save_file(&name, 0) {
                eprintln!("Failed save_state: {err}");
            }
        }

        Ok(())
    }
}

pub fn change_volume(app: &mut App, emu: &mut Emu, delta: f32) {
    emu.runtime.bus.io.apu.config.change_volume(delta);
    app.config.audio.volume = emu.runtime.bus.io.apu.config.volume;

    println!("Current volume: {}", app.config.audio.volume);
}
