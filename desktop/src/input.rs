use crate::app::{App, AppEvent};
use crate::Emu;
use core::emu::runtime::RunMode;
use core::emu::state::EmuState;
use core::emu::state::SaveStateEvent;
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, GameControllerSubsystem, Sdl};
use std::path::{Path, PathBuf};

pub struct InputHandler {
    event_pump: EventPump,
    game_controllers: Vec<GameController>,
    game_controller_subsystem: GameControllerSubsystem,
}

impl InputHandler {
    pub fn new(sdl: &Sdl) -> Result<Self, String> {
        let mut game_controllers = vec![];
        let game_controller_subsystem = sdl.game_controller()?;

        for id in 0..game_controller_subsystem.num_joysticks()? {
            if game_controller_subsystem.is_game_controller(id) {
                let controller = game_controller_subsystem.open(id).unwrap();
                game_controllers.push(controller);
            }
        }

        Ok(Self {
            event_pump: sdl.event_pump()?,
            game_controllers,
            game_controller_subsystem,
        })
    }

    /// Polls and handles events. Returns false on quit.
    pub fn handle_events(&mut self, app: &mut App, emu: &mut Emu) -> bool {
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
                    self.on_event(app, emu, AppEvent::FileDropped(filename.into()))
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(app, emu, keycode, true) {
                        self.on_event(app, emu, evt);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(app, emu, keycode, false) {
                        self.on_event(app, emu, evt);
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    if let Some(evt) = self.handle_controller_button(app, emu, button, true) {
                        self.on_event(app, emu, evt);
                    }
                }
                Event::ControllerButtonUp { button, .. } => {
                    if let Some(evt) = self.handle_controller_button(app, emu, button, false) {
                        self.on_event(app, emu, evt);
                    }
                }
                Event::JoyAxisMotion {
                    axis_idx, value, ..
                } => {
                    if let Some(evt) = self.handle_joy_axis(axis_idx, value) {
                        self.on_event(app, emu, evt);
                    }
                }
                Event::MouseButtonDown { .. } => {
                    self.on_event(app, emu, AppEvent::PickFile);
                }
                Event::Quit { .. } => return false,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    if let Some(window) = app.tiles_window.as_mut() {
                        if window.get_window_id() == window_id {
                            app.tiles_window = None;
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

    pub fn on_event(&mut self, app: &mut App, emu: &mut Emu, event: AppEvent) {
        match event {
            AppEvent::FileDropped(path) => {
                emu.load_cart_file(&path, app.config.save_state_on_exit);
                app.config.last_cart_path = path.to_str().map(|s| s.to_string());
            }
            AppEvent::Pause => {
                if emu.state == EmuState::Paused {
                    emu.state = EmuState::Running;
                } else {
                    emu.state = EmuState::Paused;
                }
            }
            AppEvent::Restart => {
                if let Some(path) = app.config.last_cart_path.clone() {
                    emu.load_cart_file(&PathBuf::from(path), false);
                }
            }
            AppEvent::ModeChanged(mode) => {
                emu.state = EmuState::Running;
                emu.runtime.set_mode(mode);
            },
            AppEvent::Mute => app.config.audio.mute = !app.config.audio.mute,
            AppEvent::SaveState(event, index) => app.handle_save_state(emu, event, index),
            AppEvent::PickFile =>
            {
                #[cfg(feature = "filepicker")]
                if emu.state == EmuState::Paused {
                    if let Some(path) = tinyfiledialogs::open_file_dialog(
                        "Select Game Boy ROM",
                        "",
                        Some((&["*.gb", "*.gbc"], "Game Boy ROMs (*.gb, *.gbc)")),
                    ) {
                        emu.load_cart_file(Path::new(&path), app.config.save_state_on_exit);
                        app.config.last_cart_path = Some(path);
                    }
                }
            }
            AppEvent::Rewind => emu.state = EmuState::Rewind,
        }
    }

    pub fn handle_controller_button(
        &mut self,
        app: &mut App,
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
                    app.next_palette(emu)
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
        app: &mut App,
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
                    app.config.interface.scale += 1.0;
                    app.set_scale(app.config.interface.scale).unwrap();
                }
            }
            Keycode::MINUS => {
                if !is_down {
                    app.config.interface.scale -= 1.0;
                    app.set_scale(app.config.interface.scale).unwrap();
                }
            }
            Keycode::F => {
                if !is_down {
                    app.toggle_fullscreen();
                }
            }
            Keycode::M => {
                if !is_down {
                    return Some(AppEvent::Mute);
                }
            }
            Keycode::P => {
                if !is_down {
                    app.next_palette(emu);
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
