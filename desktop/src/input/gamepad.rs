use crate::app::{App, AppCmd, AppState};
use crate::input::state::GamepadState;
use crate::Emu;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use sdl2::controller::Button;

pub fn handle_gamepad(
    state: &mut GamepadState,
    app: &mut App,
    emu: &mut Emu,
    button: Button,
    is_pressed: bool,
) -> Option<AppCmd> {
    state.update(button, is_pressed);
    let combo_cmd = state.handle_combo();

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
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 1));
            }
        }
        Button::RightShoulder => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 1));
            }
        }

        _ => (), // Ignore other keycodes
    }

    None
}

pub fn handle_gamepad_axis(axis_idx: u8, value: i16) -> Option<AppCmd> {
    const LEFT: u8 = 2;
    const RIGHT: u8 = 5;
    const THRESHOLD: i16 = 30_000;
    let is_pressed = value > THRESHOLD;

    if axis_idx == LEFT {
        return if is_pressed {
            Some(AppCmd::ChangeMode(RunMode::Slow))
        } else {
            Some(AppCmd::ChangeMode(RunMode::Normal))
        };
    } else if axis_idx == RIGHT {
        return if is_pressed {
            Some(AppCmd::ChangeMode(RunMode::Turbo))
        } else {
            Some(AppCmd::ChangeMode(RunMode::Normal))
        };
    }

    None
}
