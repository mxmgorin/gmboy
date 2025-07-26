use crate::app::{App, AppCmd, AppState, ChangeAppConfigCmd};
use crate::input::button::{
    handle_a, handle_b, handle_down, handle_left, handle_right, handle_select,
    handle_up,
};
use crate::Emu;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use sdl2::keyboard::Keycode;

pub fn handle_keyboard(
    app: &mut App,
    emu: &mut Emu,
    keycode: Keycode,
    is_pressed: bool,
) -> Option<AppCmd> {
    match keycode {
        Keycode::UP => handle_up(is_pressed, app, emu),
        Keycode::DOWN => handle_down(is_pressed, app, emu),
        Keycode::LEFT => return handle_left(is_pressed, app, emu),
        Keycode::RIGHT => return handle_right(is_pressed, app, emu),
        Keycode::Z => handle_b(is_pressed, app, emu),
        Keycode::X => return handle_a(is_pressed, app, emu),
        Keycode::Return => {
            if app.state == AppState::Paused && !is_pressed {
                return app.menu.select(&app.config);
            } else {
                emu.runtime.bus.io.joypad.start = is_pressed;
            }
        }
        Keycode::BACKSPACE => return handle_select(is_pressed, app, emu),
        Keycode::LCTRL | Keycode::RCTRL => {
            return if is_pressed {
                Some(AppCmd::Rewind)
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            }
        }
        Keycode::TAB => {
            return if is_pressed {
                Some(AppCmd::ChangeMode(RunMode::Turbo))
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            }
        }
        Keycode::LSHIFT | Keycode::RSHIFT => {
            return if is_pressed {
                Some(AppCmd::ChangeMode(RunMode::Slow))
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            }
        }
        Keycode::ESCAPE => {
            if !is_pressed {
                return Some(AppCmd::TogglePause);
            }
        }
        Keycode::R => {
            if !is_pressed {
                return Some(AppCmd::RestartGame);
            }
        }
        Keycode::EQUALS => {
            if !is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(1.0)));
            }
        }
        Keycode::MINUS => {
            if !is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(-1.0)));
            }
        }
        Keycode::F => {
            if !is_pressed {
                app.toggle_fullscreen();
            }
        }
        Keycode::M => {
            if !is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::ToggleMute));
            }
        }
        Keycode::F11 => {
            if !is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(-0.05)));
            }
        }
        Keycode::F12 => {
            if !is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(0.05)));
            }
        }
        Keycode::P => {
            if !is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette));
            }
        }
        Keycode::NUM_1 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 1));
            }
        }
        Keycode::NUM_2 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 2));
            }
        }
        Keycode::NUM_3 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 3));
            }
        }
        Keycode::NUM_4 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 4));
            }
        }
        Keycode::NUM_5 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 5));
            }
        }
        Keycode::NUM_6 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 6));
            }
        }
        Keycode::NUM_7 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 7));
            }
        }
        Keycode::NUM_8 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 8));
            }
        }
        Keycode::NUM_9 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 9));
            }
        }
        Keycode::F1 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 1));
            }
        }
        Keycode::F2 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 2));
            }
        }
        Keycode::F3 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 3));
            }
        }
        Keycode::F4 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 4));
            }
        }
        Keycode::F5 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 5));
            }
        }
        Keycode::F6 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 6));
            }
        }
        Keycode::F7 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 7));
            }
        }
        Keycode::F8 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 8));
            }
        }
        Keycode::F9 => {
            if !is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 9));
            }
        }
        _ => (),
    }

    None
}
