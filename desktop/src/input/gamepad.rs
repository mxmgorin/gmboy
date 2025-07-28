use crate::app::{App, AppCmd};
use crate::input::button::{
    handle_a, handle_b, handle_down, handle_left, handle_right, handle_select, handle_start,
    handle_up,
};
use crate::input::combo::ComboTracker;
use crate::Emu;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use sdl2::controller::Button;

pub fn handle_gamepad(
    state: &mut ComboTracker,
    app: &mut App,
    emu: &mut Emu,
    button: Button,
    is_pressed: bool,
) -> Option<AppCmd> {
    let combo_cmd = state.update(button, is_pressed, &app.config.input);

    if combo_cmd.is_some() {
        return combo_cmd;
    }

    match button {
        Button::Start => return handle_start(is_pressed, app, emu),
        Button::Back => return handle_select(is_pressed, app, emu),
        Button::Guide => return handle_select(is_pressed, app, emu),
        Button::DPadUp => handle_up(is_pressed, app, emu),
        Button::DPadDown => handle_down(is_pressed, app, emu),
        Button::DPadLeft => return handle_left(is_pressed, app, emu),
        Button::DPadRight => return handle_right(is_pressed, app, emu),
        Button::B => handle_b(is_pressed, app, emu),
        Button::A => return handle_a(is_pressed, app, emu),
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
        Button::LeftShoulder => {
            return if is_pressed {
                Some(AppCmd::ChangeMode(RunMode::Slow))
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            };
        }
        Button::RightShoulder => {
            return if is_pressed {
                Some(AppCmd::ChangeMode(RunMode::Turbo))
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            };
        }

        _ => (), // Ignore other keycodes
    }

    None
}

pub fn handle_gamepad_axis(app: &App, axis_idx: u8, value: i16) -> Option<AppCmd> {
    const LEFT: u8 = 2;
    const RIGHT: u8 = 5;
    const THRESHOLD: i16 = 25_000;
    let is_pressed = value > THRESHOLD;

    if axis_idx == LEFT && !is_pressed {
        return Some(AppCmd::SaveState(
            SaveStateCmd::Load,
            app.config.current_load_index,
        ));
    } else if axis_idx == RIGHT && !is_pressed {
        return Some(AppCmd::SaveState(
            SaveStateCmd::Create,
            app.config.current_save_index,
        ));
    }

    None
}
