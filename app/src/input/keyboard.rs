use crate::app::{App, AppCmd, AppState, ChangeAppConfigCmd};
use crate::input::button::{
    handle_a, handle_b, handle_down, handle_left, handle_right, handle_select, handle_start,
    handle_up,
};
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use core::emu::Emu;
use sdl2::keyboard::Keycode;

pub fn handle_keyboard(
    app: &mut App,
    emu: &mut Emu,
    keycode: Keycode,
    is_pressed: bool,
) -> Option<AppCmd> {
    log::trace!("handle_keyboard: {keycode:?}");
    
    match keycode {
        Keycode::UP => handle_up(is_pressed, app, emu),
        Keycode::DOWN => handle_down(is_pressed, app, emu),
        Keycode::LEFT => return handle_left(is_pressed, app, emu),
        Keycode::RIGHT => return handle_right(is_pressed, app, emu),
        Keycode::Z => handle_b(is_pressed, app, emu),
        Keycode::X => return handle_a(is_pressed, app, emu),
        Keycode::Return | Keycode::S => {
            return if app.state == AppState::Paused && !is_pressed {
                app.menu.select(&app.config, &*app.filesystem) // update menu for better ux
            } else {
                handle_start(is_pressed, app, emu)
            };
        }
        Keycode::BACKSPACE | Keycode::A => return handle_select(is_pressed, app, emu),
        Keycode::R => {
            return if is_pressed {
                Some(AppCmd::ToggleRewind)
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
        Keycode::SPACE => {
            return if is_pressed {
                Some(AppCmd::ChangeMode(RunMode::Slow))
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            }
        }
        Keycode::ESCAPE | Keycode::Q | Keycode::AC_BACK => {
            if is_pressed {
                return Some(AppCmd::ToggleMenu);
            }
        }
        Keycode::EQUALS => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(1.0)));
            }
        }
        Keycode::MINUS => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(-1.0)));
            }
        }
        Keycode::M => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::ToggleMute));
            }
        }
        Keycode::I => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette));
            }
        }
        Keycode::F10 => {
            if is_pressed {
                app.toggle_fullscreen();
            }
        }
        Keycode::F11 => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(-0.05)));
            }
        }
        Keycode::F12 => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(0.05)));
            }
        }
        Keycode::LEFTBRACKET => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::PrevShader));
            }
        }
        Keycode::RIGHTBRACKET => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NextShader));
            }
        }
        Keycode::P => {
            if is_pressed {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette));
            }
        }
        Keycode::NUM_1 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(1)));
            }
        }
        Keycode::NUM_2 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(2)));
            }
        }
        Keycode::NUM_3 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(3)));
            }
        }
        Keycode::NUM_4 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(4)));
            }
        }
        Keycode::NUM_5 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(5)));
            }
        }
        Keycode::NUM_6 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(6)));
            }
        }
        Keycode::NUM_7 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(7)));
            }
        }
        Keycode::NUM_8 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(8)));
            }
        }
        Keycode::NUM_9 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, Some(9)));
            }
        }
        Keycode::F1 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(1)));
            }
        }
        Keycode::F2 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(2)));
            }
        }
        Keycode::F3 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(3)));
            }
        }
        Keycode::F4 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(4)));
            }
        }
        Keycode::F5 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(5)));
            }
        }
        Keycode::F6 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(6)));
            }
        }
        Keycode::F7 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(7)));
            }
        }
        Keycode::F8 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(8)));
            }
        }
        Keycode::F9 => {
            if is_pressed {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, Some(9)));
            }
        }
        _ => (),
    }

    None
}
