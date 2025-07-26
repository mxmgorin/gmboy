use crate::app::{App, AppCmd, AppState};
use crate::Emu;

pub fn handle_start(is_pressed: bool, _app: &mut App, emu: &mut Emu) -> Option<AppCmd> {
    emu.runtime.bus.io.joypad.start = is_pressed;

    None
}

pub fn handle_select(is_pressed: bool, _app: &mut App, emu: &mut Emu) -> Option<AppCmd> {
    emu.runtime.bus.io.joypad.select = is_pressed;

    None
}

pub fn handle_up(is_pressed: bool, app: &mut App, emu: &mut Emu) {
    if app.state == AppState::Paused && !is_pressed {
        app.menu.move_up();
    } else {
        emu.runtime.bus.io.joypad.up = is_pressed;
    }
}

pub fn handle_down(is_pressed: bool, app: &mut App, emu: &mut Emu) {
    if app.state == AppState::Paused && !is_pressed {
        app.menu.move_down();
    } else {
        emu.runtime.bus.io.joypad.down = is_pressed;
    }
}

pub fn handle_left(is_pressed: bool, app: &mut App, emu: &mut Emu) -> Option<AppCmd> {
    if app.state == AppState::Paused && !is_pressed {
        return app.menu.move_left(&app.config);
    } else {
        emu.runtime.bus.io.joypad.left = is_pressed;
    }

    None
}

pub fn handle_right(is_pressed: bool, app: &mut App, emu: &mut Emu) -> Option<AppCmd> {
    if app.state == AppState::Paused && !is_pressed {
        return app.menu.move_right(&app.config);
    } else {
        emu.runtime.bus.io.joypad.right = is_pressed
    }

    None
}

pub fn handle_a(is_pressed: bool, app: &mut App, emu: &mut Emu) -> Option<AppCmd> {
    if app.state == AppState::Paused && !is_pressed {
        return app.menu.select(&app.config);
    } else {
        emu.runtime.bus.io.joypad.a = is_pressed;
    }

    None
}

pub fn handle_b(is_pressed: bool, app: &mut App, emu: &mut Emu) {
    if app.state == AppState::Paused && !is_pressed {
        app.menu.cancel();
    } else {
        emu.runtime.bus.io.joypad.b = is_pressed;
    }
}
