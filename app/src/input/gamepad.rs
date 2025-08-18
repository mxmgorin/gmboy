use crate::app::{App, AppCmd};
use crate::input::button::handle_emu_btn;
use crate::input::combos::ComboTracker;
use crate::{PlatformFileDialog, PlatformFileSystem};
use core::emu::Emu;
use sdl2::controller::Button;

pub fn handle_gamepad<FS, FD>(
    state: &mut ComboTracker,
    app: &mut App<FS, FD>,
    emu: &mut Emu,
    button: Button,
    is_pressed: bool,
) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
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

pub fn handle_gamepad_axis<FS, FD>(app: &App<FS, FD>, axis_idx: u8, value: i16) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if axis_idx == app.config.input.bindings.left_trigger.code
        && !app.config.input.bindings.left_trigger.is_pressed(value)
    {
        return app.config.input.bindings.left_trigger.cmd.clone();
    } else if axis_idx == app.config.input.bindings.right_trigger.code
        && !app.config.input.bindings.right_trigger.is_pressed(value)
    {
        return app.config.input.bindings.right_trigger.cmd.clone();
    }

    None
}
