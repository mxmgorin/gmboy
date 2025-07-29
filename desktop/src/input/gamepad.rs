use crate::app::{App, AppCmd};
use crate::input::combos::ComboTracker;
use crate::Emu;
use core::emu::state::SaveStateCmd;
use sdl2::controller::Button;
use crate::input::button::handle_emu_btn;

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

    let cmd = app.config.input.bindings.buttons.get(button, is_pressed)?;

    if let AppCmd::EmuButton(x) = cmd {
        return handle_emu_btn(*x, is_pressed, app, emu);
    }

    Some(cmd.to_owned())
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
