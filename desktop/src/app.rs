use core::emu::state::EmuState;
use core::emu::state::SaveStateEvent;
use core::emu::state::RunMode;
use crate::config::DesktopEmuConfig;
use crate::video::sdl2_tile_renderer::Sdl2TileRenderer;
use crate::video::sdl2_renderer::Sdl2Renderer;
use crate::Emu;
use core::emu::EmuCallback;
use core::into_pallet;
use core::ppu::tile::{PixelColor, TileData};
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, GameControllerSubsystem};
use std::path::{Path, PathBuf};
use crate::audio::Sdl2Audio;

pub enum AppEvent {
    Pause,
    Rewind,
    FileDropped(PathBuf),
    Restart,
    ModeChanged(RunMode),
    Mute,
    SaveState(SaveStateEvent, usize),
    PickFile,
}

pub struct App {
    _sdl_context: sdl2::Sdl,
    game_controller_subsystem: GameControllerSubsystem,
    event_pump: EventPump,
    debug_window: Option<Sdl2TileRenderer>,
    game_controllers: Vec<GameController>,
    audio: Sdl2Audio,
    pub curr_palette: [PixelColor; 4],
    pub config: DesktopEmuConfig,
    renderer: Sdl2Renderer,
}

impl EmuCallback for App {
    fn update_video(&mut self, buffer: &[u32], fps: usize) {
        self.renderer.draw_buffer(buffer);

        if self.config.graphics.show_fps {
            self.renderer.draw_fps(fps, self.curr_palette[3]);
        }

        self.renderer.present();
    }

    fn update_audio(&mut self, output: &[f32]) {
        self.audio.update(output);
    }
}

impl App {
    pub fn new(config: DesktopEmuConfig) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let scale = config.graphics.scale;
        let mut renderer = Sdl2Renderer::new(scale as u32, &video_subsystem)?;
        renderer.set_fullscreen(config.graphics.is_fullscreen);

        let debug_window = if config.graphics.tile_viewer {
            let (x, y) = renderer.position();
            let mut debug_window = Sdl2TileRenderer::new(&video_subsystem);
            debug_window.set_position(x + 640, y);

            Some(debug_window)
        } else {
            None
        };

        let mut game_controllers = vec![];
        let game_controller_subsystem = sdl_context.game_controller()?;

        for id in 0..game_controller_subsystem.num_joysticks()? {
            if game_controller_subsystem.is_game_controller(id) {
                let controller = game_controller_subsystem.open(id).unwrap();
                game_controllers.push(controller);
            }
        }

        Ok(Self {
            event_pump: sdl_context.event_pump()?,
            game_controller_subsystem,
            debug_window,
            curr_palette: into_pallet(
                &config.graphics.pallets[config.graphics.selected_pallet_idx].hex_colors,
            ),
            audio: Sdl2Audio::new(&sdl_context),
            game_controllers,
            config,
            renderer,

            _sdl_context: sdl_context,
        })
    }

    pub fn set_scale(&mut self, scale: f32) -> Result<(), String> {
        self.renderer.set_scale(scale as u32)?;

        println!("Set scale: {scale}");

        Ok(())
    }

    pub fn draw_debug(&mut self, tiles: impl Iterator<Item = TileData>) {
        if let Some(debug_window) = self.debug_window.as_mut() {
            debug_window.draw_tiles(tiles);
        }
    }

    pub fn draw_text(&mut self, lines: &[&str]) {
        let bg_color = self.curr_palette[3];
        let color = self.curr_palette[0];
        self.renderer
            .draw_text_lines(lines, self.config.graphics.text_scale, color, bg_color);

        self.renderer.present();
    }

    pub fn next_palette(&mut self, emu: &mut Emu) {
        self.config.graphics.selected_pallet_idx = get_next_pallet_idx(
            self.config.graphics.selected_pallet_idx,
            self.config.graphics.pallets.len() - 1,
        );
        let pallet = &self.config.graphics.pallets[self.config.graphics.selected_pallet_idx];
        self.curr_palette = into_pallet(&pallet.hex_colors);
        emu.runtime.bus.io.lcd.set_pallet(self.curr_palette);

        println!("Select pallet: {}", pallet.name);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.config.graphics.is_fullscreen = !self.config.graphics.is_fullscreen;
        self.renderer
            .set_fullscreen(self.config.graphics.is_fullscreen);
    }

    /// Polls and handles events. Returns false on quit.
    pub fn handle_events(&mut self, emu: &mut Emu) -> bool {
        while let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(controller) = self.game_controller_subsystem.open(which) {
                        self.game_controllers.push(controller);
                        println!("Controller {which} connected");
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    self.game_controllers.retain(|c| c.instance_id() != which);
                    println!("Controller {which} disconnected");
                }
                Event::DropFile { filename, .. } => {
                    self.on_event(emu, AppEvent::FileDropped(filename.into()))
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(emu, keycode, true) {
                        self.on_event(emu, evt);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(emu, keycode, false) {
                        self.on_event(emu, evt);
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    if let Some(evt) = self.handle_controller_button(emu, button, true) {
                        self.on_event(emu, evt);
                    }
                }
                Event::ControllerButtonUp { button, .. } => {
                    if let Some(evt) = self.handle_controller_button(emu, button, false) {
                        self.on_event(emu, evt);
                    }
                }
                Event::JoyAxisMotion {
                    axis_idx, value, ..
                } => {
                    if let Some(evt) = self.handle_joy_axis(axis_idx, value) {
                        self.on_event(emu, evt);
                    }
                }
                Event::MouseButtonDown { .. } => {
                    self.on_event(emu, AppEvent::PickFile);
                }
                Event::Quit { .. } => return false,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    if let Some(window) = self.debug_window.as_mut() {
                        if window.get_window_id() == window_id {
                            self.debug_window = None;
                        } else {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }

        true
    }

    pub fn handle_save_state(&self, emu: &mut Emu, event: SaveStateEvent, index: usize) {
        let name = self.config.get_last_cart_file_stem().unwrap();

        match event {
            SaveStateEvent::Create => {
                let save_state = emu.create_save_state();

                if let Err(err) = save_state.save_file(&name, index) {
                    eprintln!("Failed save_state: {err}",);
                }
            }
            SaveStateEvent::Load => {
                let save_state = core::emu::runtime::EmuSaveState::load_file(&name, index);

                let Ok(save_state) = save_state else {
                    eprintln!("Failed load save_state: {}", save_state.unwrap_err());
                    return;
                };

                emu.load_save_state(save_state);
            }
        }
    }

    pub fn on_event(&mut self, emu: &mut Emu, event: AppEvent) {
        match event {
            AppEvent::FileDropped(path) => {
                emu.load_cart_file(&path, self.config.save_state_on_exit);
                self.config.last_cart_path = path.to_str().map(|s| s.to_string());
            }
            AppEvent::Pause => {
                if emu.state == EmuState::Paused {
                    emu.state = EmuState::Running(RunMode::Normal);
                } else {
                    emu.state = EmuState::Paused;
                }
            }
            AppEvent::Restart => {
                if let Some(path) = self.config.last_cart_path.clone() {
                    emu.load_cart_file(&PathBuf::from(path), false);
                }
            }
            AppEvent::ModeChanged(mode) => emu.state = EmuState::Running(mode),
            AppEvent::Mute => emu.config.is_muted = !emu.config.is_muted,
            AppEvent::SaveState(event, index) => self.handle_save_state(emu, event, index),
            AppEvent::PickFile =>
            {
                #[cfg(feature = "filepicker")]
                if emu.state == EmuState::Paused {
                    if let Some(path) = tinyfiledialogs::open_file_dialog("Select ROM", "", None) {
                        emu.load_cart_file(Path::new(&path), self.config.save_state_on_exit);
                        self.config.last_cart_path = Some(path);
                    }
                }
            }
            AppEvent::Rewind => emu.state = EmuState::Rewind,
        }
    }

    pub fn handle_controller_button(
        &mut self,
        emu: &mut Emu,
        button: sdl2::controller::Button,
        is_down: bool,
    ) -> Option<AppEvent> {
        match button {
            sdl2::controller::Button::DPadUp => emu.runtime.bus.io.joypad.up = is_down,
            sdl2::controller::Button::DPadDown => emu.runtime.bus.io.joypad.down = is_down,
            sdl2::controller::Button::DPadLeft => emu.runtime.bus.io.joypad.left = is_down,
            sdl2::controller::Button::DPadRight => emu.runtime.bus.io.joypad.right = is_down,
            sdl2::controller::Button::B => emu.runtime.bus.io.joypad.b = is_down,
            sdl2::controller::Button::A => emu.runtime.bus.io.joypad.a = is_down,
            sdl2::controller::Button::Y => {
                return if is_down {
                    Some(AppEvent::Rewind)
                } else {
                    Some(AppEvent::ModeChanged(RunMode::Normal))
                }
            }
            sdl2::controller::Button::X => {
                if !is_down {
                    self.next_palette(emu)
                }
            }
            sdl2::controller::Button::Start => emu.runtime.bus.io.joypad.start = is_down,
            sdl2::controller::Button::Back => emu.runtime.bus.io.joypad.select = is_down,
            sdl2::controller::Button::Guide => emu.runtime.bus.io.joypad.select = is_down,
            sdl2::controller::Button::LeftShoulder => {
                return if is_down {
                    Some(AppEvent::ModeChanged(RunMode::Slow))
                } else {
                    Some(AppEvent::ModeChanged(RunMode::Normal))
                }
            }
            sdl2::controller::Button::RightShoulder => {
                return if is_down {
                    Some(AppEvent::ModeChanged(RunMode::Turbo))
                } else {
                    Some(AppEvent::ModeChanged(RunMode::Normal))
                }
            }

            _ => (), // Ignore other keycodes
        }

        None
    }

    pub fn handle_joy_axis(&mut self, axis_idx: u8, value: i16) -> Option<AppEvent> {
        const LEFT: u8 = 2;
        const RIGHT: u8 = 5;
        const THRESHOLD: i16 = 20_000;

        let is_down = value > THRESHOLD;

        if is_down {
            return None;
        }

        if axis_idx == LEFT {
            return Some(AppEvent::SaveState(SaveStateEvent::Load, 1));
        } else if axis_idx == RIGHT {
            return Some(AppEvent::SaveState(SaveStateEvent::Create, 1));
        }

        None
    }

    pub fn handle_key(
        &mut self,
        emu: &mut Emu,
        keycode: Keycode,
        is_down: bool,
    ) -> Option<AppEvent> {
        match keycode {
            Keycode::UP => emu.runtime.bus.io.joypad.up = is_down,
            Keycode::DOWN => emu.runtime.bus.io.joypad.down = is_down,
            Keycode::LEFT => emu.runtime.bus.io.joypad.left = is_down,
            Keycode::RIGHT => emu.runtime.bus.io.joypad.right = is_down,
            Keycode::Z => emu.runtime.bus.io.joypad.b = is_down,
            Keycode::X => emu.runtime.bus.io.joypad.a = is_down,
            Keycode::Return => emu.runtime.bus.io.joypad.start = is_down,
            Keycode::BACKSPACE => emu.runtime.bus.io.joypad.select = is_down,
            Keycode::LCTRL | Keycode::RCTRL => {
                return if is_down {
                    Some(AppEvent::Rewind)
                } else {
                    Some(AppEvent::ModeChanged(RunMode::Normal))
                }
            }
            Keycode::TAB => {
                return if is_down {
                    Some(AppEvent::ModeChanged(RunMode::Turbo))
                } else {
                    Some(AppEvent::ModeChanged(RunMode::Normal))
                }
            }
            Keycode::LSHIFT | Keycode::RSHIFT => {
                return if is_down {
                    Some(AppEvent::ModeChanged(RunMode::Slow))
                } else {
                    Some(AppEvent::ModeChanged(RunMode::Normal))
                }
            }
            Keycode::SPACE => {
                if !is_down {
                    return Some(AppEvent::Pause);
                }
            }
            Keycode::R => {
                if !is_down {
                    return Some(AppEvent::Restart);
                }
            }
            Keycode::EQUALS => {
                if !is_down {
                    self.config.graphics.scale += 1.0;
                    self.set_scale(self.config.graphics.scale).unwrap();
                }
            }
            Keycode::MINUS => {
                if !is_down {
                    self.config.graphics.scale -= 1.0;
                    self.set_scale(self.config.graphics.scale).unwrap();
                }
            }
            Keycode::F => {
                if !is_down {
                    self.toggle_fullscreen();
                }
            }
            Keycode::M => {
                if !is_down {
                    return Some(AppEvent::Mute);
                }
            }
            Keycode::P => {
                if !is_down {
                    self.next_palette(emu);
                }
            }
            Keycode::NUM_1 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 1));
                }
            }
            Keycode::NUM_2 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 2));
                }
            }
            Keycode::NUM_3 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 3));
                }
            }
            Keycode::NUM_4 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 4));
                }
            }
            Keycode::NUM_5 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 5));
                }
            }
            Keycode::NUM_6 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 6));
                }
            }
            Keycode::NUM_7 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 7));
                }
            }
            Keycode::NUM_8 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 8));
                }
            }
            Keycode::NUM_9 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Create, 9));
                }
            }
            Keycode::F1 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 1));
                }
            }
            Keycode::F2 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 2));
                }
            }
            Keycode::F3 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 3));
                }
            }
            Keycode::F4 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 4));
                }
            }
            Keycode::F5 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 5));
                }
            }
            Keycode::F6 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 6));
                }
            }
            Keycode::F7 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 7));
                }
            }
            Keycode::F8 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 8));
                }
            }
            Keycode::F9 => {
                if !is_down {
                    return Some(AppEvent::SaveState(SaveStateEvent::Load, 9));
                }
            }
            _ => (), // Ignore other keycodes
        }

        None
    }
}

pub fn get_next_pallet_idx(curr_idx: usize, max_idx: usize) -> usize {
    if curr_idx < max_idx {
        curr_idx + 1
    } else {
        0
    }
}
