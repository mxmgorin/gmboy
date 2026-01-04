use crate::app::{App, AppCmd, AppState, ChangeAppConfigCmd};
use crate::config::AppConfig;
use crate::input::emu::handle_emu_btn;
use crate::input::gamepad::GamepadHandler;
use crate::input::keyboard::handle_key;
use crate::{PlatformFileDialog, PlatformFileSystem};
use core::emu::state::EmuState;
use core::emu::Emu;
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::{EventPump, GameControllerSubsystem, Sdl};
use std::path::Path;

pub struct InputHandler {
    event_pump: EventPump,
    game_controllers: Vec<GameController>,
    game_controller_subsystem: GameControllerSubsystem,
    gamepad_handler: GamepadHandler,
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
            gamepad_handler: GamepadHandler::new(),
        })
    }

    /// Polls and handles events. Returns false on quit.
    pub fn handle_events<FS, FD>(&mut self, app: &mut App<FS, FD>, emu: &mut Emu)
    where
        FS: PlatformFileSystem,
        FD: PlatformFileDialog,
    {
        while let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(controller) = self.game_controller_subsystem.open(which) {
                        self.game_controllers.push(controller);
                        log::info!("Controller {which} connected");
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    self.game_controllers.retain(|c| c.instance_id() != which);
                    log::info!("Controller {which} disconnected");
                }
                Event::DropFile { filename, .. } => {
                    self.handle_cmd(app, emu, AppCmd::LoadFile(filename.into()))
                }
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    if let Some(evt) = handle_key(&app.config.input, sc, true) {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::KeyUp {
                    scancode: Some(sc), ..
                } => {
                    if let Some(evt) = handle_key(&app.config.input, sc, false) {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    if let Some(evt) =
                        self.gamepad_handler
                            .handle_button(&app.config.input, button, true)
                    {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::ControllerButtonUp { button, .. } => {
                    if let Some(evt) =
                        self.gamepad_handler
                            .handle_button(&app.config.input, button, false)
                    {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::JoyAxisMotion {
                    axis_idx, value, ..
                } => {
                    if let Some(evt) =
                        self.gamepad_handler
                            .handle_axis(&app.config.input, axis_idx, value)
                    {
                        self.handle_cmd(app, emu, evt);
                    }
                }
                Event::Quit { .. } => self.handle_cmd(app, emu, AppCmd::Quit),
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    if app.video.close_window(window_id) {
                        self.handle_cmd(app, emu, AppCmd::Quit);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn handle_cmd<FS, FD>(&mut self, app: &mut App<FS, FD>, emu: &mut Emu, cmd: AppCmd)
    where
        FS: PlatformFileSystem,
        FD: PlatformFileDialog,
    {
        match cmd {
            AppCmd::LoadFile(path) => {
                if let Err(err) = app.load_cart_file(emu, Path::new(&path)) {
                    log::warn!("Failed to load cart file: {err}");
                }
            }
            AppCmd::ToggleMenu => {
                if app.state == AppState::Paused && !emu.runtime.cpu.clock.bus.cart.is_empty() {
                    emu.runtime.cpu.clock.bus.io.joypad.reset();
                    app.state = AppState::Running;
                } else {
                    app.state = AppState::Paused;
                    app.menu.request_update();
                }
            }
            AppCmd::RestartRom => {
                app.restart_rom(emu);
            }
            AppCmd::ChangeMode(mode) => {
                emu.state = EmuState::Running;
                emu.runtime.set_mode(mode);
            }
            AppCmd::SaveState(event, index) => app.handle_save_state(emu, event, index),
            AppCmd::SelectRom => {
                if app.state == AppState::Paused {
                    if let Some(path) = app.platform.fd.select_file(
                        "Select Game Boy ROM",
                        (&["*.gb", "*.gbc"], "Game Boy ROMs (*.gb, *.gbc)"),
                    ) {
                        if let Err(err) = app.load_cart_file(emu, Path::new(&path)) {
                            log::warn!("Failed to load cart file: {err}");
                        }
                    }
                }
            }
            AppCmd::ToggleRewind => {
                if emu.state == EmuState::Rewind {
                    emu.state = EmuState::Running
                } else {
                    emu.state = EmuState::Rewind
                }
            }
            AppCmd::Quit => app.state = AppState::Quitting,
            AppCmd::SelectRomsDir => {
                if let Some(dir) = app.platform.fd.select_dir("Select ROMs Folder") {
                    let result = app.roms.load_from_dir(&dir, &app.platform.fs);

                    let Ok(count) = result else {
                        log::error!("Failed to load ROMs: {}", result.unwrap_err());
                        return;
                    };

                    app.notifications.add(format!("Found {count} ROMs"));
                }
            }
            AppCmd::ChangeConfig(cmd) => match cmd {
                ChangeAppConfigCmd::Volume(x) => app.change_volume(emu, x),
                ChangeAppConfigCmd::Scale(x) => app.change_scale(x).unwrap(),
                ChangeAppConfigCmd::TileWindow => {
                    app.config.video.interface.show_tiles = !app.config.video.interface.show_tiles;
                    app.video.update_config(&app.config.video);
                }
                ChangeAppConfigCmd::Fullscreen => app.toggle_fullscreen(),
                ChangeAppConfigCmd::Fps => {
                    app.config.video.interface.show_fps = !app.config.video.interface.show_fps;
                    emu.runtime
                        .cpu
                        .clock
                        .bus
                        .io
                        .ppu
                        .toggle_fps(app.config.video.interface.show_fps);
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
                    emu.runtime.cpu.clock.bus.io.apu.config.buffer_size =
                        core::change_usize(emu.runtime.cpu.clock.bus.io.apu.config.buffer_size, x)
                            .clamp(0, 2560);
                    emu.runtime.cpu.clock.bus.io.apu.update_buffer_size();
                    app.config.audio.buffer_size =
                        emu.runtime.cpu.clock.bus.io.apu.config.buffer_size;
                }
                ChangeAppConfigCmd::MuteTurbo => {
                    app.config.audio.mute_turbo = !app.config.audio.mute_turbo
                }
                ChangeAppConfigCmd::MuteSlow => {
                    app.config.audio.mute_slow = !app.config.audio.mute_slow
                }
                ChangeAppConfigCmd::Reset => {
                    app.config = AppConfig::default();
                    emu.config = app.config.emulation.clone();
                    app.notifications.add("Defaults restored");
                }
                ChangeAppConfigCmd::ComboInterval(x) => {
                    app.config.input.combo_interval =
                        core::change_duration(app.config.input.combo_interval, x);
                }
                ChangeAppConfigCmd::SetSaveIndex(x) => app.config.current_save_index = x,
                ChangeAppConfigCmd::SetLoadIndex(x) => app.config.current_load_index = x,
                ChangeAppConfigCmd::InvertPalette => {
                    app.config.video.interface.is_palette_inverted =
                        !app.config.video.interface.is_palette_inverted;
                    app.update_palette(emu);
                }
                ChangeAppConfigCmd::Video(x) => {
                    if app.config.video.render.backend != x.render.backend {
                        app.notifications.add("Restart is required to apply");
                    }

                    app.config.video = x;
                    app.video.update_config(&app.config.video);
                }
                ChangeAppConfigCmd::IncSaveAndLoadIndexes => {
                    app.config.inc_save_index();
                    app.config.inc_load_index();
                    app.notifications
                        .add(format!("Save Index: {}", app.config.current_save_index));
                    app.notifications
                        .add(format!("Load Index: {}", app.config.current_load_index));
                    app.menu.request_update();
                }
                ChangeAppConfigCmd::DecSaveAndLoadIndexes => {
                    app.config.dec_load_index();
                    app.config.dec_save_index();
                    app.notifications
                        .add(format!("Save Index: {}", app.config.current_save_index));
                    app.notifications
                        .add(format!("Load Index: {}", app.config.current_load_index));
                    app.menu.request_update();
                }
                ChangeAppConfigCmd::NextShader => app.next_shader(),
                ChangeAppConfigCmd::PrevShader => app.prev_shader(),
                ChangeAppConfigCmd::FrameSkip(x) => {
                    app.config.video.render.frame_skip = x;
                    app.video.update_config(&app.config.video);
                }
            },
            AppCmd::ReleaseButton(btn) => {
                if let Some(cmd) = handle_emu_btn(btn, false, app, emu) {
                    self.handle_cmd(app, emu, cmd);
                }
            }
            AppCmd::PressButton(btn) => {
                if let Some(cmd) = handle_emu_btn(btn, true, app, emu) {
                    self.handle_cmd(app, emu, cmd);
                }
            }
            AppCmd::SetFileBrowsePath(path) => app.roms.last_browse_dir_path = Some(path),
            AppCmd::ToggleFullscreen => app.toggle_fullscreen(),
            AppCmd::Macro(cmds) => {
                for cmd in cmds {
                    self.handle_cmd(app, emu, cmd);
                }
            }
            AppCmd::BindKeyboard(key_name, btn) => {
                if let Some(sc) = sdl2::keyboard::Scancode::from_name(&key_name) {
                    app.config.input.bindings.keys.bind_btn(sc, btn);
                } else {
                    log::warn!("Failed to bind key: invalid name {key_name}");
                }
            }
        }
    }
}
