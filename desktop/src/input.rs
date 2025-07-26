use crate::app::{change_volume, App, AppCmd, AppState, ChangeAppConfigCmd};
use crate::config::AppConfig;
use crate::video::menu::AppMenu;
use crate::Emu;
use core::emu::runtime::RunMode;
use core::emu::state::EmuState;
use core::emu::state::SaveStateCmd;
use sdl2::controller::{Button, GameController};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, GameControllerSubsystem, Sdl};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const COMBO_WINDOW: Duration = Duration::from_millis(100);

pub struct ButtonState {
    pub pressed: bool,
    pub last_pressed: Instant,
    pub button: Button,
}

impl ButtonState {
    fn new(button: Button) -> Self {
        Self {
            button,
            pressed: false,
            last_pressed: Instant::now(),
        }
    }

    fn update(&mut self, is_pressed: bool) {
        if is_pressed && !self.pressed {
            self.last_pressed = Instant::now();
        }

        self.pressed = is_pressed;
    }
}

pub struct GamepadState {
    states: Vec<ButtonState>,
}

impl GamepadState {
    pub fn new() -> Self {
        Self {
            states: vec![
                ButtonState::new(Button::Start),
                ButtonState::new(Button::Back),
                ButtonState::new(Button::Guide),
            ],
        }
    }

    pub fn update(&mut self, button: Button, pressed: bool) {
        for state in self.states.iter_mut() {
            if state.button == button {
                state.update(pressed);
                break;
            }
        }
    }

    pub fn get_combo_cmd(&self) -> Option<AppCmd> {
        if self.combo_2(Button::Back, Button::Start) || self.combo_2(Button::Guide, Button::Start) {
            return Some(AppCmd::TogglePause);
        }

        None
    }

    /// Generic function to check any 2-button combo
    pub fn combo_2(&self, b1: Button, b2: Button) -> bool {
        let mut state_1: Option<&ButtonState> = None;
        let mut state_2: Option<&ButtonState> = None;

        for state in &self.states {
            if state.button == b1 {
                state_1 = Some(state);
            } else if state.button == b2 {
                state_2 = Some(state);
            }
        }

        let (s1, s2) = match (state_1, state_2) {
            (Some(a), Some(b)) => (a, b),
            _ => return false,
        };

        if s1.pressed && s2.pressed {
            let diff = if s1.last_pressed > s2.last_pressed {
                s1.last_pressed.duration_since(s2.last_pressed)
            } else {
                s2.last_pressed.duration_since(s1.last_pressed)
            };

            return diff <= COMBO_WINDOW;
        }

        false
    }
}

pub struct InputHandler {
    event_pump: EventPump,
    game_controllers: Vec<GameController>,
    game_controller_subsystem: GameControllerSubsystem,
    gamepad_state: GamepadState,
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
            gamepad_state: GamepadState::new(),
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
                    self.execute_command(app, emu, AppCmd::LoadFile(filename.into()))
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
                Event::Quit { .. } => self.execute_command(app, emu, AppCmd::Quit),
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    if let Some(window) = app.tile_window.as_mut() {
                        if window.get_window_id() == window_id {
                            app.toggle_tile_window();
                        } else {
                            self.execute_command(app, emu, AppCmd::Quit);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn execute_command(&mut self, app: &mut App, emu: &mut Emu, event: AppCmd) {
        match event {
            AppCmd::LoadFile(path) => {
                emu.load_cart_file(&path, app.config.save_state_on_exit);
                app.config.last_cart_path = path.to_str().map(|s| s.to_string());
                app.state = AppState::Running;
                app.menu = AppMenu::new(!emu.runtime.bus.cart.is_empty());
            }
            AppCmd::TogglePause => {
                if app.state == AppState::Paused && !emu.runtime.bus.cart.is_empty() {
                    emu.runtime.bus.io.joypad.reset();
                    app.state = AppState::Running;
                } else {
                    app.state = AppState::Paused;
                }
            }
            AppCmd::RestartGame => {
                if let Some(path) = app.config.last_cart_path.clone() {
                    emu.load_cart_file(&PathBuf::from(path), false);

                    app.state = AppState::Running;
                }
            }
            AppCmd::ChangeMode(mode) => {
                emu.state = EmuState::Running;
                emu.runtime.set_mode(mode);
            }
            AppCmd::SaveState(event, index) => app.handle_save_state(emu, event, index),
            AppCmd::PickFile =>
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
            AppCmd::Rewind => emu.state = EmuState::Rewind,
            AppCmd::Quit => app.state = AppState::Quitting,
            AppCmd::ChangeConfig(cmd) => match cmd {
                ChangeAppConfigCmd::Volume(x) => change_volume(app, emu, x),
                ChangeAppConfigCmd::Scale(x) => app.change_scale(x).unwrap(),
                ChangeAppConfigCmd::TileWindow => app.toggle_tile_window(),
                ChangeAppConfigCmd::Fullscreen => app.toggle_fullscreen(),
                ChangeAppConfigCmd::Fps => {
                    emu.runtime.ppu.toggle_fps();
                    app.config.interface.show_fps = !app.config.interface.show_fps;
                }
                ChangeAppConfigCmd::SpinDuration(x) => {
                    emu.config.spin_duration = core::change_duration(emu.config.spin_duration, x);
                    app.config.emulation.spin_duration = emu.config.spin_duration;
                }
                ChangeAppConfigCmd::NextPalette => app.next_palette(emu),
                ChangeAppConfigCmd::PrevPalette => app.prev_palette(emu),
                ChangeAppConfigCmd::ToggleMute => app.config.audio.mute = !app.config.audio.mute,
                ChangeAppConfigCmd::NormalSpeed(x) => {
                    emu.config.normal_speed =
                        core::change_f64_rounded(emu.config.normal_speed, x as f64).max(0.05);
                    app.config.emulation.normal_speed = emu.config.normal_speed;
                }
                ChangeAppConfigCmd::TurboSpeed(x) => {
                    emu.config.turbo_speed =
                        core::change_f64_rounded(emu.config.turbo_speed, x as f64).max(0.05);
                    app.config.emulation.turbo_speed = emu.config.turbo_speed;
                }
                ChangeAppConfigCmd::SlowSpeed(x) => {
                    emu.config.slow_speed =
                        core::change_f64_rounded(emu.config.slow_speed, x as f64).max(0.05);
                    app.config.emulation.slow_speed = emu.config.slow_speed;
                }
                ChangeAppConfigCmd::RewindSize(x) => {
                    emu.config.rewind_size =
                        core::change_usize(emu.config.rewind_size, x).clamp(0, 500);
                    app.config.emulation.rewind_size = emu.config.rewind_size;
                }
                ChangeAppConfigCmd::RewindInterval(x) => {
                    emu.config.rewind_interval =
                        core::change_duration(emu.config.rewind_interval, x);
                    app.config.emulation.rewind_interval = emu.config.rewind_interval;
                }
                ChangeAppConfigCmd::SaveStateOnExit => {
                    app.config.save_state_on_exit = !app.config.save_state_on_exit
                }
                ChangeAppConfigCmd::AudioBufferSize(x) => {
                    emu.runtime.bus.io.apu.config.buffer_size =
                        core::change_usize(emu.runtime.bus.io.apu.config.buffer_size, x)
                            .clamp(0, 2560);
                    emu.runtime.bus.io.apu.update_buffer_size();
                    app.config.audio.buffer_size = emu.runtime.bus.io.apu.config.buffer_size;
                }
                ChangeAppConfigCmd::MuteTurbo => {
                    app.config.audio.mute_turbo = !app.config.audio.mute_turbo
                }
                ChangeAppConfigCmd::MuteSlow => {
                    app.config.audio.mute_slow = !app.config.audio.mute_slow
                }
                ChangeAppConfigCmd::Default => {
                    app.config = AppConfig::default();
                    emu.config = app.config.emulation.clone();
                }
            },
        }
    }

    pub fn handle_controller_button(
        &mut self,
        app: &mut App,
        emu: &mut Emu,
        button: Button,
        is_pressed: bool,
    ) -> Option<AppCmd> {
        self.gamepad_state.update(button, is_pressed);
        let combo_cmd = self.gamepad_state.get_combo_cmd();

        if combo_cmd.is_some() {
            return combo_cmd;
        }

        match button {
            Button::DPadUp => {
                if app.state == AppState::Paused && !is_pressed {
                    app.menu.move_up();
                } else {
                    emu.runtime.bus.io.joypad.up = is_pressed;
                }
            }
            Button::DPadDown => {
                if app.state == AppState::Paused && !is_pressed {
                    app.menu.move_down();
                } else {
                    emu.runtime.bus.io.joypad.down = is_pressed;
                }
            }
            Button::DPadLeft => {
                if app.state == AppState::Paused && !is_pressed {
                    return app.menu.move_left();
                } else {
                    emu.runtime.bus.io.joypad.left = is_pressed
                }
            }
            Button::DPadRight => {
                if app.state == AppState::Paused && !is_pressed {
                    return app.menu.move_right();
                } else {
                    emu.runtime.bus.io.joypad.right = is_pressed
                }
            }
            Button::B => {
                if app.state == AppState::Paused && !is_pressed {
                    app.menu.cancel();
                } else {
                    emu.runtime.bus.io.joypad.b = is_pressed
                }
            }
            Button::A => {
                if app.state == AppState::Paused && !is_pressed {
                    return app.menu.select();
                } else {
                    emu.runtime.bus.io.joypad.a = is_pressed
                }
            }
            Button::Y => {
                return if is_pressed {
                    Some(AppCmd::Rewind)
                } else {
                    Some(AppCmd::ChangeMode(RunMode::Normal))
                }
            }
            Button::X => {
                if !is_pressed {
                    app.next_palette(emu)
                }
            }
            Button::Start => emu.runtime.bus.io.joypad.start = is_pressed,
            Button::Back => emu.runtime.bus.io.joypad.select = is_pressed,
            Button::Guide => emu.runtime.bus.io.joypad.select = is_pressed,
            Button::LeftShoulder => {
                return if is_pressed {
                    Some(AppCmd::ChangeMode(RunMode::Slow))
                } else {
                    Some(AppCmd::ChangeMode(RunMode::Normal))
                }
            }
            Button::RightShoulder => {
                return if is_pressed {
                    Some(AppCmd::ChangeMode(RunMode::Turbo))
                } else {
                    Some(AppCmd::ChangeMode(RunMode::Normal))
                }
            }

            _ => (), // Ignore other keycodes
        }

        None
    }

    pub fn handle_joy_axis(&mut self, axis_idx: u8, value: i16) -> Option<AppCmd> {
        const LEFT: u8 = 2;
        const RIGHT: u8 = 5;
        const THRESHOLD: i16 = 20_000;

        let is_down = value > THRESHOLD;

        if is_down {
            return None;
        }

        if axis_idx == LEFT {
            return Some(AppCmd::SaveState(SaveStateCmd::Load, 1));
        } else if axis_idx == RIGHT {
            return Some(AppCmd::SaveState(SaveStateCmd::Create, 1));
        }

        None
    }

    pub fn handle_key(
        &mut self,
        app: &mut App,
        emu: &mut Emu,
        keycode: Keycode,
        is_down: bool,
    ) -> Option<AppCmd> {
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
                    Some(AppCmd::Rewind)
                } else {
                    Some(AppCmd::ChangeMode(RunMode::Normal))
                }
            }
            Keycode::TAB => {
                return if is_down {
                    Some(AppCmd::ChangeMode(RunMode::Turbo))
                } else {
                    Some(AppCmd::ChangeMode(RunMode::Normal))
                }
            }
            Keycode::LSHIFT | Keycode::RSHIFT => {
                return if is_down {
                    Some(AppCmd::ChangeMode(RunMode::Slow))
                } else {
                    Some(AppCmd::ChangeMode(RunMode::Normal))
                }
            }
            Keycode::ESCAPE => {
                if !is_down {
                    return Some(AppCmd::TogglePause);
                }
            }
            Keycode::R => {
                if !is_down {
                    return Some(AppCmd::RestartGame);
                }
            }
            Keycode::EQUALS => {
                if !is_down {
                    return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(1.0)));
                }
            }
            Keycode::MINUS => {
                if !is_down {
                    return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(-1.0)));
                }
            }
            Keycode::F => {
                if !is_down {
                    app.toggle_fullscreen();
                }
            }
            Keycode::M => {
                if !is_down {
                    return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::ToggleMute));
                }
            }
            Keycode::F11 => {
                if !is_down {
                    return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(-0.05)));
                }
            }
            Keycode::F12 => {
                if !is_down {
                    return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(0.05)));
                }
            }
            Keycode::P => {
                if !is_down {
                    return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette));
                }
            }
            Keycode::NUM_1 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 1));
                }
            }
            Keycode::NUM_2 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 2));
                }
            }
            Keycode::NUM_3 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 3));
                }
            }
            Keycode::NUM_4 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 4));
                }
            }
            Keycode::NUM_5 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 5));
                }
            }
            Keycode::NUM_6 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 6));
                }
            }
            Keycode::NUM_7 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 7));
                }
            }
            Keycode::NUM_8 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 8));
                }
            }
            Keycode::NUM_9 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Create, 9));
                }
            }
            Keycode::F1 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 1));
                }
            }
            Keycode::F2 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 2));
                }
            }
            Keycode::F3 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 3));
                }
            }
            Keycode::F4 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 4));
                }
            }
            Keycode::F5 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 5));
                }
            }
            Keycode::F6 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 6));
                }
            }
            Keycode::F7 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 7));
                }
            }
            Keycode::F8 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 8));
                }
            }
            Keycode::F9 => {
                if !is_down {
                    return Some(AppCmd::SaveState(SaveStateCmd::Load, 9));
                }
            }
            _ => (), // Ignore other keycodes
        }

        None
    }
}
