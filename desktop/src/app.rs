use crate::audio::AppAudio;
use crate::config::AppConfig;
use crate::input::handler::InputHandler;
use crate::video::game_window::GameWindow;
use crate::video::menu::AppMenu;
use crate::video::notification::Notifications;
use crate::video::tiles_window::TileWindow;
use crate::Emu;
use core::emu::battery::BatterySave;
use core::emu::runtime::EmuRuntime;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use core::emu::EmuCallback;
use core::ppu::palette::LcdPalette;
use sdl2::{Sdl, VideoSubsystem};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

pub const AUTO_SAVE_STATE_SUFFIX: &str = "auto";

pub enum AppCmd {
    TogglePause,
    Rewind,
    LoadFile(PathBuf),
    RestartGame,
    ChangeMode(RunMode),
    SaveState(SaveStateCmd, usize),
    PickFile,
    Quit,
    ChangeConfig(ChangeAppConfigCmd),
}

pub enum ChangeAppConfigCmd {
    Default,
    Volume(f32),
    Scale(f32),
    TileWindow,
    Fullscreen,
    Fps,
    SpinDuration(i32),
    NextPalette,
    PrevPalette,
    ToggleMute,
    NormalSpeed(f32),
    TurboSpeed(f32),
    SlowSpeed(f32),
    RewindSize(i32),
    RewindInterval(i32),
    AutoSaveState,
    AudioBufferSize(i32),
    MuteTurbo,
    MuteSlow,
    ComboInterval(i32),
    SaveIndex(usize),
    LoadIndex(usize),
    PaletteInverted,
    FrameBlendAlpha(f32),
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
    pub window: GameWindow,
    palettes: Box<[LcdPalette]>,
    pub tile_window: Option<TileWindow>,
    pub state: AppState,
    pub config: AppConfig,
    pub menu: AppMenu,
    notifications: Notifications,
}

impl EmuCallback for App {
    fn update_video(&mut self, buffer: &[u32], runtime: &EmuRuntime) {
        self.window.draw_buffer(buffer);
        self.draw_notification(runtime.ppu.get_fps());

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
        let palette = config.interface.get_palette_colors(&palettes);
        let video_subsystem = sdl.video()?;
        let mut game_window = GameWindow::new(
            config.interface.scale as u32,
            &video_subsystem,
            palette[0],
            palette[3],
            config.interface.frame_blend_alpha
        )?;
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
            notifications: Notifications::new(Duration::from_secs(3)),
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
            if self.state == AppState::Paused {
                self.run_pause(emu, input);
            } else {
                input.handle_events(self, emu);
                emu.run_frame(self)?;

                if let Some(tiles_window) = self.tile_window.as_mut() {
                    tiles_window.draw_tiles(emu.runtime.bus.video_ram.iter_tiles());
                }
            }
        }

        Ok(())
    }

    pub fn run_pause(&mut self, emu: &mut Emu, input: &mut InputHandler) {
        while self.state == AppState::Paused {
            input.handle_events(self, emu);
            emu.runtime.clock.reset();
            self.draw_menu();
            self.draw_notification(None);

            self.window.present();
            thread::sleep(Duration::from_millis(30));
        }
    }

    pub fn draw_notification(&mut self, fps: Option<&str>) {
        let lines = self.notifications.update_and_get();

        if lines.is_empty() {
            if let Some(fps) = &fps {
                self.window.draw_notification(&[fps]);
            }
        } else {
            self.window.draw_notification(lines);
        }
    }

    pub fn change_scale(&mut self, delta: f32) -> Result<(), String> {
        self.config.interface.scale = (self.config.interface.scale + delta).max(0.0);
        self.window.set_scale(self.config.interface.scale as u32)?;
        let msg = format!("Scale: {}", self.config.interface.scale);
        self.notifications.add(msg);

        Ok(())
    }

    fn draw_menu(&mut self) {
        let items = self.menu.get_items(&self.config);
        let lines: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
        self.window.draw_text_lines(&lines, true, true);
    }

    pub fn next_palette(&mut self, emu: &mut Emu) {
        self.config.interface.selected_palette_idx = core::move_next_wrapped(
            self.config.interface.selected_palette_idx,
            self.palettes.len() - 1,
        );
        self.update_palette(emu);
    }

    pub fn prev_palette(&mut self, emu: &mut Emu) {
        self.config.interface.selected_palette_idx = core::move_prev_wrapped(
            self.config.interface.selected_palette_idx,
            self.palettes.len() - 1,
        );
        self.update_palette(emu);
    }

    pub fn update_palette(&mut self, emu: &mut Emu) {
        let palette = &self.palettes[self.config.interface.selected_palette_idx];
        let colors = self.config.interface.get_palette_colors(&self.palettes);
        self.window.text_color = colors[0];
        self.window.bg_color = colors[3];
        emu.runtime.bus.io.lcd.set_pallet(colors);

        let suffix = if self.config.interface.is_palette_inverted {
            " (inverted)"
        } else {
            ""
        };
        let msg = format!("Palette: {}{}", palette.name, suffix);
        self.notifications.add(msg);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.config.interface.is_fullscreen = !self.config.interface.is_fullscreen;
        self.window
            .set_fullscreen(self.config.interface.is_fullscreen);
    }

    pub fn handle_save_state(&mut self, emu: &mut Emu, event: SaveStateCmd, index: usize) {
        let name = self.config.get_last_file_stem().unwrap();
        let index = index.to_string();

        match event {
            SaveStateCmd::Create => {
                let save_state = emu.create_save_state();

                if let Err(err) = save_state.save_file(&name, &index) {
                    eprintln!("Failed save_state: {err}");
                    return;
                }

                let msg = format!("Saved save state: {index}");
                self.notifications.add(msg);
            }
            SaveStateCmd::Load => {
                let save_state = core::emu::runtime::EmuSaveState::load_file(&name, &index);

                let Ok(save_state) = save_state else {
                    eprintln!("Failed load save_state: {}", save_state.unwrap_err());
                    return;
                };

                emu.load_save_state(save_state);
                let msg = format!("Loaded save state: {index}");
                self.notifications.add(msg);
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

        let name = self.config.get_last_file_stem();

        let Some(name) = name else {
            return Err("Failed get_last_file_stem: not found".to_string());
        };

        // save sram for battery emulation
        if let Some(bytes) = emu.runtime.bus.cart.dump_ram() {
            let battery = BatterySave::from_bytes(bytes)
                .save_file(&name)
                .map_err(|e| e.to_string());

            if let Err(err) = battery {
                eprint!("Failed BatterySave: {err}");
            };
        }

        if self.config.auto_save_state {
            if let Err(err) = emu
                .create_save_state()
                .save_file(&name, AUTO_SAVE_STATE_SUFFIX)
            {
                eprintln!("Failed save_state: {err}");
            }
        }

        Ok(())
    }

    pub fn load_cart_file(&mut self, emu: &mut Emu, path: &Path) {
        let path_str = path.to_str().map(|s| s.to_string());
        let is_reload = self.config.last_cart_path == path_str && !emu.runtime.bus.cart.is_empty();
        self.config.last_cart_path = path_str;
        emu.load_cart_file(path);
        self.state = AppState::Running;
        self.menu = AppMenu::new(!emu.runtime.bus.cart.is_empty());

        if !is_reload && self.config.auto_save_state {
            let name = path.file_stem().unwrap().to_str().expect("cart is valid");
            let save_state =
                core::emu::state::EmuSaveState::load_file(name, AUTO_SAVE_STATE_SUFFIX);

            if let Ok(save_state) = save_state {
                emu.load_save_state(save_state);
            } else {
                eprintln!("Failed load save_state: {}", save_state.unwrap_err());
            };
        }
    }

    pub fn change_volume(&mut self, emu: &mut Emu, delta: f32) {
        emu.runtime.bus.io.apu.config.change_volume(delta);
        self.config.audio.volume = emu.runtime.bus.io.apu.config.volume;

        let msg = format!("Volume: {}", self.config.audio.volume * 100.0);
        self.notifications.add(msg);
    }
}
