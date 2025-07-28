use crate::app::{App, AppCmd, AppState, ChangeAppConfigCmd};
use crate::config::{AppConfig, InputConfig};
use crate::input::combo::ComboTracker;
use crate::input::gamepad::{handle_gamepad, handle_gamepad_axis};
use crate::input::keyboard::handle_keyboard;
use crate::roms::RomsList;
use crate::Emu;
use core::emu::state::EmuState;
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::{EventPump, GameControllerSubsystem, Sdl};
use std::path::{Path, PathBuf};

pub struct InputHandler {
    event_pump: EventPump,
    game_controllers: Vec<GameController>,
    game_controller_subsystem: GameControllerSubsystem,
    combo_tracker: ComboTracker,
}

impl InputHandler {
    pub fn new(sdl: &Sdl, config: &InputConfig) -> Result<Self, String> {
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
            combo_tracker: ComboTracker::new(config.combo_interval),
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
                    self.handle_cmd(app, emu, AppCmd::LoadFile(filename.into()))
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = handle_keyboard(app, emu, keycode, true) {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = handle_keyboard(app, emu, keycode, false) {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    if let Some(evt) =
                        handle_gamepad(&mut self.combo_tracker, app, emu, button, true)
                    {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::ControllerButtonUp { button, .. } => {
                    if let Some(evt) =
                        handle_gamepad(&mut self.combo_tracker, app, emu, button, false)
                    {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::JoyAxisMotion {
                    axis_idx, value, ..
                } => {
                    if let Some(evt) = handle_gamepad_axis(axis_idx, value) {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::Quit { .. } => self.handle_cmd(app, emu, AppCmd::Quit),
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    if let Some(window) = app.tile_window.as_mut() {
                        if window.get_window_id() == window_id {
                            app.toggle_tile_window();
                        } else {
                            self.handle_cmd(app, emu, AppCmd::Quit);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn handle_cmd(&mut self, app: &mut App, emu: &mut Emu, event: AppCmd) {
        match event {
            AppCmd::LoadFile(path) => {
                app.load_cart_file(emu, &path);
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
                if let Some(path) = RomsList::get_or_create().get_last_path() {
                    app.load_cart_file(emu, &PathBuf::from(path));
                }
            }
            AppCmd::ChangeMode(mode) => {
                emu.state = EmuState::Running;
                emu.runtime.set_mode(mode);
            }
            AppCmd::SaveState(event, index) => app.handle_save_state(emu, event, index),
            AppCmd::SelectRom =>
            {
                #[cfg(feature = "filepicker")]
                if app.state == AppState::Paused {
                    if let Some(path) = tinyfiledialogs::open_file_dialog(
                        "Select Game Boy ROM",
                        "",
                        Some((&["*.gb", "*.gbc"], "Game Boy ROMs (*.gb, *.gbc)")),
                    ) {
                        app.load_cart_file(emu, Path::new(&path));
                    }
                }
            }
            AppCmd::Rewind => emu.state = EmuState::Rewind,
            AppCmd::Quit => app.state = AppState::Quitting,
            AppCmd::SelectRomsDir => {
                if let Some(dir) = tinyfiledialogs::select_folder_dialog("Select ROMs Folder", "") {
                    let mut lib = RomsList::get_or_create();
                    let result = lib.load_from_dir(&dir);

                    let Ok(count) = result else {
                        eprintln!("Failed to load ROMs library: {}", result.unwrap_err());
                        return;
                    };

                    if let Err(err) = core::save_json_file(&RomsList::get_path(), &lib) {
                        eprintln!("Failed to save ROMs library: {err}");
                    }

                    app.config.roms_dir = Some(dir);
                    app.notifications.add(format!("Found {count} ROMs"));
                }
            }
            AppCmd::ChangeConfig(cmd) => match cmd {
                ChangeAppConfigCmd::Volume(x) => app.change_volume(emu, x),
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
                ChangeAppConfigCmd::AutoSaveState => {
                    app.config.auto_save_state = !app.config.auto_save_state
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
                ChangeAppConfigCmd::ComboInterval(x) => {
                    app.config.input.combo_interval =
                        core::change_duration(app.config.input.combo_interval, x);
                    self.combo_tracker = ComboTracker::new(app.config.input.combo_interval);
                }
                ChangeAppConfigCmd::SaveIndex(x) => app.config.current_save_index = x,
                ChangeAppConfigCmd::LoadIndex(x) => app.config.current_load_index = x,
                ChangeAppConfigCmd::PaletteInverted => {
                    app.config.interface.is_palette_inverted =
                        !app.config.interface.is_palette_inverted;
                    app.update_palette(emu);
                }
                ChangeAppConfigCmd::Video(x) => {
                    app.config.video = x;
                    app.window.config = app.config.video.clone();
                }
            },
        }
    }
}
