use crate::app::{App, AppCmd, AppState, ChangeAppConfigCmd};
use crate::Emu;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use sdl2::keyboard::Keycode;

pub fn handle_keyboard(
    app: &mut App,
    emu: &mut Emu,
    keycode: Keycode,
    is_down: bool,
) -> Option<AppCmd> {
    match keycode {
        Keycode::UP => {
            if app.state == AppState::Paused && !is_down {
                app.menu.move_up();
            } else {
                emu.runtime.bus.io.joypad.up = is_down;
            }
        }
        Keycode::DOWN => {
            if app.state == AppState::Paused && !is_down {
                app.menu.move_down();
            } else {
                emu.runtime.bus.io.joypad.down = is_down;
            }
        }
        Keycode::LEFT => {
            if app.state == AppState::Paused && !is_down {
                return app.menu.move_left();
            } else {
                emu.runtime.bus.io.joypad.left = is_down
            }
        }
        Keycode::RIGHT => {
            if app.state == AppState::Paused && !is_down {
                return app.menu.move_right();
            } else {
                emu.runtime.bus.io.joypad.right = is_down
            }
        }
        Keycode::Z => emu.runtime.bus.io.joypad.b = is_down,
        Keycode::X => emu.runtime.bus.io.joypad.a = is_down,
        Keycode::Return => {
            if app.state == AppState::Paused && !is_down {
                return app.menu.select();
            } else {
                emu.runtime.bus.io.joypad.start = is_down;
            }
        }
        Keycode::BACKSPACE => {
            if app.state == AppState::Paused && !is_down {
                app.menu.cancel();
            } else {
                emu.runtime.bus.io.joypad.select = is_down
            }
        }
        Keycode::LCTRL | Keycode::RCTRL => {
            return if is_down {
                Some(AppCmd::Rewind)
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            }
        }
        Keycode::TAB => {
            return if is_down {
                Some(AppCmd::ChangeMode(RunMode::Turbo))
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            }
        }
        Keycode::LSHIFT | Keycode::RSHIFT => {
            return if is_down {
                Some(AppCmd::ChangeMode(RunMode::Slow))
            } else {
                Some(AppCmd::ChangeMode(RunMode::Normal))
            }
        }
        Keycode::ESCAPE => {
            if !is_down {
                return Some(AppCmd::TogglePause);
            }
        }
        Keycode::R => {
            if !is_down {
                return Some(AppCmd::RestartGame);
            }
        }
        Keycode::EQUALS => {
            if !is_down {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(1.0)));
            }
        }
        Keycode::MINUS => {
            if !is_down {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(-1.0)));
            }
        }
        Keycode::F => {
            if !is_down {
                app.toggle_fullscreen();
            }
        }
        Keycode::M => {
            if !is_down {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::ToggleMute));
            }
        }
        Keycode::F11 => {
            if !is_down {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(-0.05)));
            }
        }
        Keycode::F12 => {
            if !is_down {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(0.05)));
            }
        }
        Keycode::P => {
            if !is_down {
                return Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette));
            }
        }
        Keycode::NUM_1 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 1));
            }
        }
        Keycode::NUM_2 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 2));
            }
        }
        Keycode::NUM_3 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 3));
            }
        }
        Keycode::NUM_4 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 4));
            }
        }
        Keycode::NUM_5 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 5));
            }
        }
        Keycode::NUM_6 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 6));
            }
        }
        Keycode::NUM_7 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 7));
            }
        }
        Keycode::NUM_8 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 8));
            }
        }
        Keycode::NUM_9 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Create, 9));
            }
        }
        Keycode::F1 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 1));
            }
        }
        Keycode::F2 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 2));
            }
        }
        Keycode::F3 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 3));
            }
        }
        Keycode::F4 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 4));
            }
        }
        Keycode::F5 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 5));
            }
        }
        Keycode::F6 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 6));
            }
        }
        Keycode::F7 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 7));
            }
        }
        Keycode::F8 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 8));
            }
        }
        Keycode::F9 => {
            if !is_down {
                return Some(AppCmd::SaveState(SaveStateCmd::Load, 9));
            }
        }
        _ => (), // Ignore other keycodes
    }

    None
}
