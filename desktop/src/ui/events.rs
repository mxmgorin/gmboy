use crate::ui::Ui;
use crate::Emu;
use core::emu::state::{RunMode, EmuState};
use core::emu::state::SaveStateEvent;
use sdl2::keyboard::Keycode;
use std::path::{Path, PathBuf};

pub enum UiEvent {
    Pause,
    Rewind,
    FileDropped(PathBuf),
    Restart,
    ModeChanged(RunMode),
    Mute,
    SaveState(SaveStateEvent, usize),
    PickFile,
}

impl Ui {
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

    pub fn on_event(&mut self, emu: &mut Emu, event: UiEvent) {
        match event {
            UiEvent::FileDropped(path) => {
                emu.load_cart_file(&path, self.config.save_state_on_exit);
                self.config.last_cart_path = path.to_str().map(|s| s.to_string());
            }
            UiEvent::Pause => {
                if emu.state == EmuState::Paused {
                    emu.state = EmuState::Running(RunMode::Normal);
                } else {
                    emu.state = EmuState::Paused;
                }
            }
            UiEvent::Restart => {
                if let Some(path) = self.config.last_cart_path.clone() {
                    emu.load_cart_file(&PathBuf::from(path), false);
                }
            }
            UiEvent::ModeChanged(mode) => emu.state = EmuState::Running(mode),
            UiEvent::Mute => emu.config.is_muted = !emu.config.is_muted,
            UiEvent::SaveState(event, index) => self.handle_save_state(emu, event, index),
            UiEvent::PickFile => {
                if emu.state == EmuState::Paused {
                    if let Some(path) = tinyfiledialogs::open_file_dialog("Select ROM", "", None) {
                        emu.load_cart_file(Path::new(&path), self.config.save_state_on_exit);
                        self.config.last_cart_path = Some(path);
                    }
                }
            }
            UiEvent::Rewind => emu.state = EmuState::Rewind,
        }
    }

    pub fn handle_controller_button(
        &mut self,
        emu: &mut Emu,
        button: sdl2::controller::Button,
        is_down: bool,
    ) -> Option<UiEvent> {
        match button {
            sdl2::controller::Button::DPadUp => emu.runtime.bus.io.joypad.up = is_down,
            sdl2::controller::Button::DPadDown => emu.runtime.bus.io.joypad.down = is_down,
            sdl2::controller::Button::DPadLeft => emu.runtime.bus.io.joypad.left = is_down,
            sdl2::controller::Button::DPadRight => emu.runtime.bus.io.joypad.right = is_down,
            sdl2::controller::Button::B => emu.runtime.bus.io.joypad.b = is_down,
            sdl2::controller::Button::A => emu.runtime.bus.io.joypad.a = is_down,
            sdl2::controller::Button::Y => {
                return if is_down {
                    Some(UiEvent::Rewind)
                } else {
                    Some(UiEvent::ModeChanged(RunMode::Normal))
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
                    Some(UiEvent::ModeChanged(RunMode::Slow))
                } else {
                    Some(UiEvent::ModeChanged(RunMode::Normal))
                }
            }
            sdl2::controller::Button::RightShoulder => {
                return if is_down {
                    Some(UiEvent::ModeChanged(RunMode::Turbo))
                } else {
                    Some(UiEvent::ModeChanged(RunMode::Normal))
                }
            }

            _ => (), // Ignore other keycodes
        }

        None
    }

    pub fn handle_joy_axis(&mut self, axis_idx: u8, value: i16) -> Option<UiEvent> {
        const LEFT: u8 = 2;
        const RIGHT: u8 = 5;
        const THRESHOLD: i16 = 20_000;

        let is_down = value > THRESHOLD;

        if is_down {
            return None;
        }

        if axis_idx == LEFT {
            return Some(UiEvent::SaveState(SaveStateEvent::Load, 1));
        } else if axis_idx == RIGHT {
            return Some(UiEvent::SaveState(SaveStateEvent::Create, 1));
        }

        None
    }

    pub fn handle_key(
        &mut self,
        emu: &mut Emu,
        keycode: Keycode,
        is_down: bool,
    ) -> Option<UiEvent> {
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
                    Some(UiEvent::Rewind)
                } else {
                    Some(UiEvent::ModeChanged(RunMode::Normal))
                }
            }
            Keycode::TAB => {
                return if is_down {
                    Some(UiEvent::ModeChanged(RunMode::Turbo))
                } else {
                    Some(UiEvent::ModeChanged(RunMode::Normal))
                }
            }
            Keycode::LSHIFT | Keycode::RSHIFT => {
                return if is_down {
                    Some(UiEvent::ModeChanged(RunMode::Slow))
                } else {
                    Some(UiEvent::ModeChanged(RunMode::Normal))
                }
            }
            Keycode::SPACE => {
                if !is_down {
                    return Some(UiEvent::Pause);
                }
            }
            Keycode::R => {
                if !is_down {
                    return Some(UiEvent::Restart);
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
                    return Some(UiEvent::Mute);
                }
            }
            Keycode::P => {
                if !is_down {
                    self.next_palette(emu);
                }
            }
            Keycode::NUM_1 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 1));
                }
            }
            Keycode::NUM_2 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 2));
                }
            }
            Keycode::NUM_3 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 3));
                }
            }
            Keycode::NUM_4 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 4));
                }
            }
            Keycode::NUM_5 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 5));
                }
            }
            Keycode::NUM_6 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 6));
                }
            }
            Keycode::NUM_7 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 7));
                }
            }
            Keycode::NUM_8 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 8));
                }
            }
            Keycode::NUM_9 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Create, 9));
                }
            }
            Keycode::F1 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 1));
                }
            }
            Keycode::F2 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 2));
                }
            }
            Keycode::F3 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 3));
                }
            }
            Keycode::F4 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 4));
                }
            }
            Keycode::F5 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 5));
                }
            }
            Keycode::F6 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 6));
                }
            }
            Keycode::F7 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 7));
                }
            }
            Keycode::F8 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 8));
                }
            }
            Keycode::F9 => {
                if !is_down {
                    return Some(UiEvent::SaveState(SaveStateEvent::Load, 9));
                }
            }
            _ => (), // Ignore other keycodes
        }

        None
    }
}
