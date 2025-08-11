use crate::audio::AppAudio;
use crate::battery::BatterySave;
use crate::config::{AppConfig, VideoBackendType, VideoConfig};
use crate::input::handler::InputHandler;
use crate::menu::AppMenu;
use crate::notification::Notifications;
use crate::palette::LcdPalette;
use crate::roms::RomsState;
use crate::video::shader::{next_shader_by_name, prev_shader_by_name};
use crate::video::AppVideo;
use crate::{AppConfigFile, AppPlatform, PlatformFileDialog, PlatformFileSystem};
use core::auxiliary::joypad::JoypadButton;
use core::cart::Cart;
use core::emu::runtime::EmuRuntime;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use core::emu::Emu;
use core::emu::EmuAudioCallback;
use sdl2::Sdl;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

pub const AUTO_SAVE_STATE_SUFFIX: &str = "auto";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppCmd {
    ToggleMenu,
    ToggleRewind,
    LoadFile(PathBuf),
    RestartGame,
    ChangeMode(RunMode),
    SaveState(SaveStateCmd, Option<usize>),
    SelectRom,
    Quit,
    ChangeConfig(ChangeAppConfigCmd),
    SelectRomsDir,
    EmuButton(JoypadButton),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChangeAppConfigCmd {
    Reset,
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
    SetSaveIndex(usize),
    SetLoadIndex(usize),
    IncSaveAndLoadIndexes,
    DecSaveAndLoadIndexes,
    InvertPalette,
    Video(VideoConfig),
    NextShader,
    PrevShader,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AppState {
    Paused,
    Running,
    Quitting,
}

pub struct App<FS, FD>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    audio: AppAudio,
    palettes: Box<[LcdPalette]>,
    pub video: AppVideo,
    pub state: AppState,
    pub config: AppConfig,
    pub menu: AppMenu,
    pub notifications: Notifications,
    pub platform: AppPlatform<FS, FD>,
}

impl<FS, FD> EmuAudioCallback for App<FS, FD>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    fn update(&mut self, output: &[f32], runtime: &EmuRuntime) {
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

impl<FS, FD> App<FS, FD>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    pub fn new(
        sdl: &mut Sdl,
        mut config: AppConfig,
        palettes: Box<[LcdPalette]>,
        platform: AppPlatform<FS, FD>,
    ) -> Result<Self, String> {
        let colors = config.video.interface.get_palette_colors(&palettes);
        let mut notifications = Notifications::new(Duration::from_secs(3));

        let video = AppVideo::new(sdl, colors[0], colors[3], &config.video);
        let video = match video {
            Ok(video) => video,
            Err(err) => {
                log::error!("Failed to init AppVideo: {err}");
                if config.video.render.backend == VideoBackendType::Gl {
                    let msg = "GL init failed, fallback to SDL2";
                    log::info!("{msg}");
                    notifications.add(msg);
                    config.video.render.backend = VideoBackendType::Sdl2;

                    AppVideo::new(sdl, colors[0], colors[3], &config.video)?
                } else {
                    return Err(err);
                }
            }
        };
        let roms = RomsState::get_or_create(&platform.fs);

        Ok(Self {
            audio: AppAudio::new(sdl, &config.audio),
            video,
            menu: AppMenu::new(roms.get_last_path().is_some()),
            state: AppState::Paused,
            palettes,
            config,
            notifications,
            platform,
        })
    }

    /// Execution loop
    pub fn run(&mut self, emu: &mut Emu, input: &mut InputHandler) -> Result<(), String> {
        self.state = if self.config.auto_continue && !emu.runtime.bus.cart.is_empty() {
            AppState::Running
        } else {
            AppState::Paused
        };

        const MIN_RENDER_INTERVAL: Duration = Duration::from_millis(33); // ~30 FPS
        let mut last_render_time = Instant::now();

        while self.state != AppState::Quitting {
            if self.state == AppState::Paused {
                self.update_pause(emu, input);
            } else {
                input.handle_events(self, emu);
                let on_time = emu.run_frame(self)?;
                let now = Instant::now();
                let time_since_last_render = now.duration_since(last_render_time);

                if on_time || time_since_last_render >= MIN_RENDER_INTERVAL {
                    self.video.draw_buffer(&emu.runtime.ppu.pipeline.buffer);
                    self.draw_notification(emu.runtime.ppu.get_fps());
                    self.video.show();
                    last_render_time = now;
                }
            }
        }

        Ok(())
    }

    pub fn update_pause(&mut self, emu: &mut Emu, input: &mut InputHandler) {
        input.handle_events(self, emu);
        emu.runtime.clock.reset();
        self.draw_menu();
        self.draw_notification(None);
        self.video.show();
        thread::sleep(Duration::from_millis(30));
    }

    pub fn draw_notification(&mut self, fps: Option<(&str, bool)>) {
        let (lines, updated) = self.notifications.update_and_get();

        if lines.is_empty() {
            if let Some((fps, updated)) = fps {
                if updated {
                    self.video.ui.update_fps(fps);
                }

                self.video.draw_fps();
            }
        } else if updated || !lines.is_empty() {
            if updated {
                self.video.ui.update_notif(lines);
            }

            self.video.draw_notif();
        }
    }

    pub fn change_scale(&mut self, delta: f32) -> Result<(), String> {
        self.config.video.interface.scale = (self.config.video.interface.scale + delta).max(0.0);
        self.video
            .set_scale(self.config.video.interface.scale as u32)?;
        let msg = format!("Scale: {}", self.config.video.interface.scale);
        self.notifications.add(msg);

        Ok(())
    }

    fn draw_menu(&mut self) {
        let (items, updated) = self.menu.get_items(&self.config);

        if updated {
            self.video.ui.update_menu(items, true, true);
        }

        self.video.draw_menu();
    }

    pub fn next_palette(&mut self, emu: &mut Emu) {
        self.config.video.interface.selected_palette_idx = core::move_next_wrapped(
            self.config.video.interface.selected_palette_idx,
            self.palettes.len() - 1,
        );
        self.update_palette(emu);
    }

    pub fn prev_palette(&mut self, emu: &mut Emu) {
        self.config.video.interface.selected_palette_idx = core::move_prev_wrapped(
            self.config.video.interface.selected_palette_idx,
            self.palettes.len() - 1,
        );
        self.update_palette(emu);
    }

    pub fn update_palette(&mut self, emu: &mut Emu) {
        let palette = &self.palettes[self.config.video.interface.selected_palette_idx];
        let colors = self
            .config
            .video
            .interface
            .get_palette_colors(&self.palettes);
        self.video.ui.text_color = colors[0];
        self.video.ui.bg_color = colors[3];
        emu.runtime.bus.io.lcd.set_pallet(colors);
        self.menu.request_update();

        let suffix = if self.config.video.interface.is_palette_inverted {
            " (inverted)"
        } else {
            ""
        };
        let msg = format!("Palette: {}{}", palette.name, suffix);
        self.notifications.add(msg);
    }

    pub fn next_shader(&mut self) {
        let (name, _shader) = next_shader_by_name(&self.config.video.render.gl.shader_name);
        self.update_shader(name);
    }

    pub fn prev_shader(&mut self) {
        let (name, _shader) = prev_shader_by_name(&self.config.video.render.gl.shader_name);
        self.update_shader(name);
    }

    pub fn update_shader(&mut self, name: impl Into<String>) {
        self.config.video.render.gl.shader_name = name.into();
        self.video.update_config(&self.config.video);
        self.menu.request_update();
        self.notifications.add(format!(
            "Shader: {}",
            self.config.video.render.gl.shader_name
        ));
    }

    pub fn toggle_fullscreen(&mut self) {
        self.config.video.interface.is_fullscreen = !self.config.video.interface.is_fullscreen;
        self.video
            .set_fullscreen(self.config.video.interface.is_fullscreen);
    }

    pub fn handle_save_state(&mut self, emu: &mut Emu, event: SaveStateCmd, index: Option<usize>) {
        let roms = RomsState::get_or_create(&self.platform.fs);
        let path = roms.get_last_path().unwrap();
        let name = self.platform.fs.get_file_name(path).unwrap();

        match event {
            SaveStateCmd::Create => {
                let save_state = emu.create_save_state();
                let index = index.unwrap_or(self.config.current_save_index).to_string();

                if let Err(err) = AppConfigFile::write_save_state_file(&save_state, &name, &index) {
                    log::error!("Failed save_state: {err}");
                    return;
                }

                let msg = format!("Saved save state: {index}");
                self.notifications.add(msg);
            }
            SaveStateCmd::Load => {
                let index = index.unwrap_or(self.config.current_load_index).to_string();
                let save_state = AppConfigFile::read_save_state_file(&name, &index);

                let Ok(save_state) = save_state else {
                    log::error!("Failed load save_state: {}", save_state.unwrap_err());
                    return;
                };

                emu.load_save_state(save_state);
                emu.runtime.bus.io.lcd.apply_colors(
                    self.config
                        .video
                        .interface
                        .get_palette_colors(&self.palettes),
                );
                emu.runtime.bus.io.apu.config = self.config.audio.get_apu_config();

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
            log::warn!("Failed config.save: {err}");
        }

        let roms = RomsState::get_or_create(&self.platform.fs);
        let path = roms.get_last_path();
        
        let Some(path) = path else {
            return Ok(());
        };
        
        let name = self.platform.fs.get_file_name(path);

        let Some(name) = name else {
            return Err("Failed filesystem.get_file_name: not found".to_string());
        };

        // save sram for battery emulation
        if let Some(bytes) = emu.runtime.bus.cart.dump_ram() {
            let battery = BatterySave::from_bytes(bytes)
                .save_file(&name)
                .map_err(|e| e.to_string());

            if let Err(err) = battery {
                log::warn!("Failed BatterySave: {err}");
            };
        }

        if self.config.auto_save_state {
            let state = emu.create_save_state();
            if let Err(err) =
                AppConfigFile::write_save_state_file(&state, &name, AUTO_SAVE_STATE_SUFFIX)
            {
                log::warn!("Failed save_state: {err}");
            }
        }

        Ok(())
    }

    pub fn load_cart_file(&mut self, emu: &mut Emu, path: &Path) -> Result<(), String> {
        let mut roms = RomsState::get_or_create(&self.platform.fs);
        let is_reload = roms.get_last_path().map(|x| x.as_path()) == Some(path)
            && !emu.runtime.bus.cart.is_empty();
        let file_name = self
            .platform
            .fs
            .get_file_name(path)
            .ok_or("filesystem.get_file_name: None")?;
        let ram_bytes = BatterySave::load_file(&file_name).ok().map(|x| x.ram_bytes);
        let cart_bytes = self
            .platform
            .fs
            .read_file_bytes(path)
            .ok_or("filesystem.read_file_bytes: None")?;
        let mut cart = Cart::new(cart_bytes).map_err(|e| e.to_string())?;
        _ = core::print_cart(&cart).map_err(|e| log::error!("Failed print_cart: {e}"));

        if let Some(ram_bytes) = ram_bytes {
            cart.load_ram(ram_bytes);
        }

        emu.load_cart(cart);
        roms.on_opened(path.to_path_buf());
        roms.save_file();

        emu.runtime.bus.io.lcd.apply_colors(
            self.config
                .video
                .interface
                .get_palette_colors(&self.palettes),
        );
        emu.runtime.bus.io.apu.config = self.config.audio.get_apu_config();
        self.state = AppState::Running;
        self.menu = AppMenu::new(!emu.runtime.bus.cart.is_empty());

        if !is_reload && self.config.auto_save_state {
            let path = roms.get_last_path().unwrap();
            let name = self.platform.fs.get_file_name(path).unwrap();
            let save_state = AppConfigFile::read_save_state_file(&name, AUTO_SAVE_STATE_SUFFIX);

            if let Ok(save_state) = save_state {
                emu.load_save_state(save_state);
            } else {
                log::warn!("Failed load save_state: {}", save_state.unwrap_err());
            };
        }

        Ok(())
    }

    pub fn change_volume(&mut self, emu: &mut Emu, delta: f32) {
        emu.runtime.bus.io.apu.config.change_volume(delta);
        self.config.audio.volume = emu.runtime.bus.io.apu.config.volume;

        let msg = format!("Volume: {}", self.config.audio.volume * 100.0);
        self.notifications.add(msg);
    }
}
