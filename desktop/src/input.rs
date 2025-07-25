use crate::app::{change_volume, App, AppCommand, AppState, ChangeAppConfigCommand};
use crate::video::menu::AppMenu;
use crate::Emu;
use core::emu::runtime::RunMode;
use core::emu::state::EmuState;
use core::emu::state::SaveStateCommand;
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, GameControllerSubsystem, Sdl};
use std::ops::{Add, Sub};
use std::path::{Path, PathBuf};
use std::time::Duration;

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
    pub fn handle_events(&mut self, app: &mut App, emu: &mut Emu) {
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
                    self.execute_command(app, emu, AppCommand::LoadFile(filename.into()))
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(app, emu, keycode, true) {
                        self.execute_command(app, emu, evt);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(app, emu, keycode, false) {
                        self.execute_command(app, emu, evt);
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    if let Some(evt) = self.handle_controller_button(app, emu, button, true) {
                        self.execute_command(app, emu, evt);
                    }
                }
                Event::ControllerButtonUp { button, .. } => {
                    if let Some(evt) = self.handle_controller_button(app, emu, button, false) {
                        self.execute_command(app, emu, evt);
                    }
                }
                Event::JoyAxisMotion {
                    axis_idx, value, ..
                } => {
                    if let Some(evt) = self.handle_joy_axis(axis_idx, value) {
                        self.execute_command(app, emu, evt);
                    }
                }
                Event::MouseButtonDown { .. } => {
                    self.execute_command(app, emu, AppCommand::PickFile);
                }
                Event::Quit { .. } => self.execute_command(app, emu, AppCommand::Quit),
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    if let Some(window) = app.tile_window.as_mut() {
                        if window.get_window_id() == window_id {
                            app.toggle_tile_window();
                        } else {
                            self.execute_command(app, emu, AppCommand::Quit);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn execute_command(&mut self, app: &mut App, emu: &mut Emu, event: AppCommand) {
        match event {
            AppCommand::LoadFile(path) => {
                emu.load_cart_file(&path, app.config.save_state_on_exit);
                app.config.last_cart_path = path.to_str().map(|s| s.to_string());
                app.state = AppState::Running;
                app.menu = AppMenu::new(!emu.runtime.bus.cart.is_empty());
            }
            AppCommand::TogglePause => {
                if app.state == AppState::Paused && !emu.runtime.bus.cart.is_empty() {
                    app.state = AppState::Running;
                } else {
                    app.state = AppState::Paused;
                }
            }
            AppCommand::Restart => {
                if let Some(path) = app.config.last_cart_path.clone() {
                    emu.load_cart_file(&PathBuf::from(path), false);
                }
            }
            AppCommand::ChangeMode(mode) => {
                emu.state = EmuState::Running;
                emu.runtime.set_mode(mode);
            }
            AppCommand::SaveState(event, index) => app.handle_save_state(emu, event, index),
            AppCommand::PickFile =>
            {
                #[cfg(feature = "filepicker")]
                if app.state == AppState::Paused {
                    if let Some(path) = tinyfiledialogs::open_file_dialog(
                        "Select Game Boy ROM",
                        "",
                        Some((&["*.gb", "*.gbc"], "Game Boy ROMs (*.gb, *.gbc)")),
                    ) {
                        emu.load_cart_file(Path::new(&path), app.config.save_state_on_exit);
                        app.config.last_cart_path = Some(path);
                        app.state = AppState::Running;
                        app.menu = AppMenu::new(!emu.runtime.bus.cart.is_empty());
                    }
                }
            }
            AppCommand::Rewind => emu.state = EmuState::Rewind,
            AppCommand::Quit => app.state = AppState::Quitting,
            AppCommand::ChangeConfig(cmd) => match cmd {
                ChangeAppConfigCommand::Volume(x) => change_volume(app, emu, x),
                ChangeAppConfigCommand::Scale(x) => app.change_scale(x).unwrap(),
                ChangeAppConfigCommand::TileWindow => app.toggle_tile_window(),
                ChangeAppConfigCommand::Fullscreen => app.toggle_fullscreen(),
                ChangeAppConfigCommand::Fps => {
                    emu.runtime.ppu.toggle_fps();
                    app.config.interface.show_fps = !app.config.interface.show_fps;
                }
                ChangeAppConfigCommand::SpinDuration(x) => {
                    if x < 0 {
                        emu.config.spin_duration = emu
                            .config
                            .spin_duration
                            .sub(Duration::from_nanos(x.unsigned_abs() as u64));
                    } else {
                        emu.config.spin_duration =
                            emu.config.spin_duration.add(Duration::from_nanos(x as u64));
                    }
                    app.config.emulation.spin_duration = emu.config.spin_duration;
                }
                ChangeAppConfigCommand::NextPalette => app.next_palette(emu),
                ChangeAppConfigCommand::ToggleMute => {
                    app.config.audio.mute = !app.config.audio.mute
                }
                ChangeAppConfigCommand::NormalSpeed(x) => {
                    emu.config.normal_speed = core::add_f64_rounded(emu.config.normal_speed, x as f64);
                    app.config.emulation.normal_speed = emu.config.normal_speed;
                }
                ChangeAppConfigCommand::TurboSpeed(x) => {
                    emu.config.turbo_speed = core::add_f64_rounded(emu.config.turbo_speed, x as f64);
                    app.config.emulation.turbo_speed = emu.config.turbo_speed;
                }
                ChangeAppConfigCommand::SlowSpeed(x) => {
                    emu.config.slow_speed = core::add_f64_rounded(emu.config.slow_speed, x as f64);
                    app.config.emulation.slow_speed = emu.config.slow_speed;
                }
                ChangeAppConfigCommand::RewindSize(x) => {
                    if x < 0 {
                        emu.config.rewind_size -= x as usize;
                    } else {
                        emu.config.rewind_size += x as usize;
                    }
                    app.config.emulation.rewind_size = emu.config.rewind_size;
                }
                ChangeAppConfigCommand::RewindInterval(x) => {
                    if x < 0 {
                        emu.config.rewind_interval = emu
                            .config
                            .rewind_interval
                            .sub(Duration::from_nanos(x.unsigned_abs() as u64));
                    } else {
                        emu.config.rewind_interval = emu
                            .config
                            .rewind_interval
                            .add(Duration::from_nanos(x as u64));
                    }
                    app.config.emulation.rewind_interval = emu.config.rewind_interval;
                }
                ChangeAppConfigCommand::SaveStateOnExit => {
                    app.config.save_state_on_exit = !app.config.save_state_on_exit
                }
                ChangeAppConfigCommand::AudioBufferSize(x) => {
                    if x < 0 {
                        emu.runtime.bus.io.apu.config.buffer_size -= x.unsigned_abs() as usize;
                    } else {
                        emu.runtime.bus.io.apu.config.buffer_size += x as usize;
                    }
                    emu.runtime.bus.io.apu.update_buffer_size();
                    app.config.audio.buffer_size = emu.runtime.bus.io.apu.config.buffer_size;
                }
                ChangeAppConfigCommand::MuteTurbo => app.config.audio.mute_turbo = !app.config.audio.mute_turbo,
                ChangeAppConfigCommand::MuteSlow => app.config.audio.mute_slow = !app.config.audio.mute_slow,
            },
        }
    }

    pub fn handle_controller_button(
        &mut self,
        app: &mut App,
        emu: &mut Emu,
        button: sdl2::controller::Button,
        is_down: bool,
    ) -> Option<AppCommand> {
        match button {
            sdl2::controller::Button::DPadUp => {
                if app.state == AppState::Paused && !is_down {
                    app.menu.move_up();
                } else {
                    emu.runtime.bus.io.joypad.up = is_down;
                }
            }
            sdl2::controller::Button::DPadDown => {
                if app.state == AppState::Paused && !is_down {
                    app.menu.move_down();
                } else {
                    emu.runtime.bus.io.joypad.down = is_down;
                }
            }
            sdl2::controller::Button::DPadLeft => {
                if app.state == AppState::Paused && !is_down {
                    return app.menu.move_left();
                } else {
                    emu.runtime.bus.io.joypad.left = is_down
                }
            }
            sdl2::controller::Button::DPadRight => {
                if app.state == AppState::Paused && !is_down {
                    return app.menu.move_right();
                } else {
                    emu.runtime.bus.io.joypad.right = is_down
                }
            }
            sdl2::controller::Button::B => {
                if app.state == AppState::Paused && !is_down {
                    app.menu.cancel();
                } else {
                    emu.runtime.bus.io.joypad.b = is_down
                }
            }
            sdl2::controller::Button::A => {
                if app.state == AppState::Paused && !is_down {
                    return app.menu.select();
                } else {
                    emu.runtime.bus.io.joypad.a = is_down
                }
            }
            sdl2::controller::Button::Y => {
                return if is_down {
                    Some(AppCommand::Rewind)
                } else {
                    Some(AppCommand::ChangeMode(RunMode::Normal))
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
                    Some(AppCommand::ChangeMode(RunMode::Slow))
                } else {
                    Some(AppCommand::ChangeMode(RunMode::Normal))
                }
            }
            sdl2::controller::Button::RightShoulder => {
                return if is_down {
                    Some(AppCommand::ChangeMode(RunMode::Turbo))
                } else {
                    Some(AppCommand::ChangeMode(RunMode::Normal))
                }
            }

            _ => (), // Ignore other keycodes
        }

        None
    }

    pub fn handle_joy_axis(&mut self, axis_idx: u8, value: i16) -> Option<AppCommand> {
        const LEFT: u8 = 2;
        const RIGHT: u8 = 5;
        const THRESHOLD: i16 = 20_000;

        let is_down = value > THRESHOLD;

        if is_down {
            return None;
        }

        if axis_idx == LEFT {
            return Some(AppCommand::SaveState(SaveStateCommand::Load, 1));
        } else if axis_idx == RIGHT {
            return Some(AppCommand::SaveState(SaveStateCommand::Create, 1));
        }

        None
    }

    pub fn handle_key(
        &mut self,
        app: &mut App,
        emu: &mut Emu,
        keycode: Keycode,
        is_down: bool,
    ) -> Option<AppCommand> {
        match keycode {
            Keycode::UP => {
                if app.state == AppState::Paused && !is_down {
                    app.menu.move_up();
                } else {
                    emu.runtime.bus.io.joypad.up = is_down;
                }
            }
            Keycode::DOWN => {
                if app.state == AppState::Paused && !is_down {
                    app.menu.move_down();
                } else {
                    emu.runtime.bus.io.joypad.down = is_down;
                }
            }
            Keycode::LEFT => {
                if app.state == AppState::Paused && !is_down {
                    return app.menu.move_left();
                } else {
                    emu.runtime.bus.io.joypad.left = is_down
                }
            }
            Keycode::RIGHT => {
                if app.state == AppState::Paused && !is_down {
                    return app.menu.move_right();
                } else {
                    emu.runtime.bus.io.joypad.right = is_down
                }
            }
            Keycode::Z => emu.runtime.bus.io.joypad.b = is_down,
            Keycode::X => emu.runtime.bus.io.joypad.a = is_down,
            Keycode::Return => {
                if app.state == AppState::Paused && !is_down {
                    return app.menu.select();
                } else {
                    emu.runtime.bus.io.joypad.start = is_down;
                }
            }
            Keycode::BACKSPACE => {
                if app.state == AppState::Paused && !is_down {
                    app.menu.cancel();
                } else {
                    emu.runtime.bus.io.joypad.select = is_down
                }
            }
            Keycode::LCTRL | Keycode::RCTRL => {
                return if is_down {
                    Some(AppCommand::Rewind)
                } else {
                    Some(AppCommand::ChangeMode(RunMode::Normal))
                }
            }
            Keycode::TAB => {
                return if is_down {
                    Some(AppCommand::ChangeMode(RunMode::Turbo))
                } else {
                    Some(AppCommand::ChangeMode(RunMode::Normal))
                }
            }
            Keycode::LSHIFT | Keycode::RSHIFT => {
                return if is_down {
                    Some(AppCommand::ChangeMode(RunMode::Slow))
                } else {
                    Some(AppCommand::ChangeMode(RunMode::Normal))
                }
            }
            Keycode::ESCAPE => {
                if !is_down {
                    return Some(AppCommand::TogglePause);
                }
            }
            Keycode::R => {
                if !is_down {
                    return Some(AppCommand::Restart);
                }
            }
            Keycode::EQUALS => {
                if !is_down {
                    return Some(AppCommand::ChangeConfig(ChangeAppConfigCommand::Scale(1.0)));
                }
            }
            Keycode::MINUS => {
                if !is_down {
                    return Some(AppCommand::ChangeConfig(ChangeAppConfigCommand::Scale(
                        -1.0,
                    )));
                }
            }
            Keycode::F => {
                if !is_down {
                    app.toggle_fullscreen();
                }
            }
            Keycode::M => {
                if !is_down {
                    return Some(AppCommand::ChangeConfig(ChangeAppConfigCommand::ToggleMute));
                }
            }
            Keycode::F11 => {
                if !is_down {
                    return Some(AppCommand::ChangeConfig(ChangeAppConfigCommand::Volume(
                        -0.05,
                    )));
                }
            }
            Keycode::F12 => {
                if !is_down {
                    return Some(AppCommand::ChangeConfig(ChangeAppConfigCommand::Volume(
                        0.05,
                    )));
                }
            }
            Keycode::P => {
                if !is_down {
                    return Some(AppCommand::ChangeConfig(
                        ChangeAppConfigCommand::NextPalette,
                    ));
                }
            }
            Keycode::NUM_1 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 1));
                }
            }
            Keycode::NUM_2 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 2));
                }
            }
            Keycode::NUM_3 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 3));
                }
            }
            Keycode::NUM_4 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 4));
                }
            }
            Keycode::NUM_5 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 5));
                }
            }
            Keycode::NUM_6 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 6));
                }
            }
            Keycode::NUM_7 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 7));
                }
            }
            Keycode::NUM_8 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 8));
                }
            }
            Keycode::NUM_9 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Create, 9));
                }
            }
            Keycode::F1 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 1));
                }
            }
            Keycode::F2 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 2));
                }
            }
            Keycode::F3 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 3));
                }
            }
            Keycode::F4 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 4));
                }
            }
            Keycode::F5 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 5));
                }
            }
            Keycode::F6 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 6));
                }
            }
            Keycode::F7 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 7));
                }
            }
            Keycode::F8 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 8));
                }
            }
            Keycode::F9 => {
                if !is_down {
                    return Some(AppCommand::SaveState(SaveStateCommand::Load, 9));
                }
            }
            _ => (), // Ignore other keycodes
        }

        None
    }
}
