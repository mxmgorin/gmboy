use crate::ui::Ui;
use crate::Emu;
use core::emu::config::GraphicsConfig;
use core::emu::ctx::EmuState;
use core::emu::ctx::RunMode;
use core::emu::save_state::SaveStateEvent;
use sdl2::keyboard::Keycode;
use std::path::PathBuf;

pub enum UiEvent {
    Quit,
    Pause,
    FileDropped(PathBuf),
    Restart,
    ConfigChanged(GraphicsConfig),
    ModeChanged(RunMode),
    Mute,
    SaveState(SaveStateEvent, usize),
    PickFile,
}

impl Ui {
    pub fn on_event(&mut self, emu: &mut Emu, event: UiEvent) {
        match event {
            UiEvent::Quit => emu.ctx.state = EmuState::Quit,
            UiEvent::FileDropped(path) => emu.load_cart_file(path),
            UiEvent::Pause => {
                if emu.ctx.state == EmuState::Paused {
                    emu.ctx.state = EmuState::Running(RunMode::Normal);
                } else {
                    emu.ctx.state = EmuState::Paused;
                }
            }
            UiEvent::Restart => {
                if let Some(path) = &emu.ctx.config.last_cart_path {
                    emu.load_cart_file(PathBuf::from(path));
                }
            }
            UiEvent::ConfigChanged(config) => emu.ctx.config.graphics = config,
            UiEvent::ModeChanged(mode) => emu.ctx.state = EmuState::Running(mode),
            UiEvent::Mute => emu.ctx.config.emulation.is_muted = !emu.ctx.config.emulation.is_muted,
            UiEvent::SaveState(event, index) => emu.ctx.pending_save_state = Some((event, index)),
            UiEvent::PickFile => {
                if emu.ctx.state == EmuState::Paused {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        emu.load_cart_file(path)
                    }
                }
            }
        }
    }

    pub fn handle_controller_button(
        &mut self,
        emu: &mut Emu,
        button: sdl2::controller::Button,
        is_down: bool,
    ) -> Option<UiEvent> {
        match button {
            sdl2::controller::Button::DPadUp => emu.cpu.bus.io.joypad.up = is_down,
            sdl2::controller::Button::DPadDown => emu.cpu.bus.io.joypad.down = is_down,
            sdl2::controller::Button::DPadLeft => emu.cpu.bus.io.joypad.left = is_down,
            sdl2::controller::Button::DPadRight => emu.cpu.bus.io.joypad.right = is_down,
            sdl2::controller::Button::B => emu.cpu.bus.io.joypad.b = is_down,
            sdl2::controller::Button::A => emu.cpu.bus.io.joypad.a = is_down,
            sdl2::controller::Button::Y => {
                return if is_down {
                    Some(UiEvent::ModeChanged(RunMode::Rewind))
                } else {
                    Some(UiEvent::ModeChanged(RunMode::Normal))
                }
            }
            sdl2::controller::Button::X => {
                if !is_down {
                    self.next_palette(emu)
                }
            }
            sdl2::controller::Button::Start => emu.cpu.bus.io.joypad.start = is_down,
            sdl2::controller::Button::Back => emu.cpu.bus.io.joypad.select = is_down,
            sdl2::controller::Button::Guide => emu.cpu.bus.io.joypad.select = is_down,
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

        if axis_idx == LEFT {
            if !is_down {
                return Some(UiEvent::SaveState(SaveStateEvent::Load, 1));
            }
        } else if axis_idx == RIGHT {
            if !is_down {
                return Some(UiEvent::SaveState(SaveStateEvent::Create, 1));
            }
        }

        None
    }

    pub fn handle_key(&mut self, emu: &mut Emu, keycode: Keycode, is_down: bool) -> Option<UiEvent> {
        match keycode {
            Keycode::UP => emu.cpu.bus.io.joypad.up = is_down,
            Keycode::DOWN => emu.cpu.bus.io.joypad.down = is_down,
            Keycode::LEFT => emu.cpu.bus.io.joypad.left = is_down,
            Keycode::RIGHT => emu.cpu.bus.io.joypad.right = is_down,
            Keycode::Z => emu.cpu.bus.io.joypad.b = is_down,
            Keycode::X => emu.cpu.bus.io.joypad.a = is_down,
            Keycode::Return => emu.cpu.bus.io.joypad.start = is_down,
            Keycode::BACKSPACE => emu.cpu.bus.io.joypad.select = is_down,
            Keycode::LCTRL | Keycode::RCTRL => {
                return if is_down {
                    Some(UiEvent::ModeChanged(RunMode::Rewind))
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
                    self.set_scale(emu.ctx.config.graphics.scale + 1.0, &mut emu.ctx.config.graphics).unwrap();
                    return Some(UiEvent::ConfigChanged(emu.ctx.config.graphics.clone()));
                }
            }
            Keycode::MINUS => {
                if !is_down {
                    self.set_scale(emu.ctx.config.graphics.scale - 1.0, &mut emu.ctx.config.graphics).unwrap();
                    return Some(UiEvent::ConfigChanged(emu.ctx.config.graphics.clone()));
                }
            }
            Keycode::F => {
                if !is_down {
                    self.toggle_fullscreen(&mut emu.ctx.config.graphics);
                    return Some(UiEvent::ConfigChanged(emu.ctx.config.graphics.clone()));
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
                    return Some(UiEvent::ConfigChanged(emu.ctx.config.graphics.clone()));
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
